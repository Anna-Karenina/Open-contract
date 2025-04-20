function transformFields(fields) {
  return fields.reduce((result, field) => {
    if (
      field.type === "object" &&
      field.inner_fields.length === 0 &&
      !field.is_required
    ) {
      return result;
    }

    const fieldDef = {
      type: field.type,
      is_required: field.is_required,
    };

    if (field.type === "object" && field.inner_fields.length > 0) {
      const nested = transformFields(field.inner_fields);
      result[field.key] = field.is_repeated ? [nested] : nested;
    } else {
      result[field.key] = field.is_repeated ? [fieldDef] : fieldDef;
    }

    return result;
  }, {});
}
let apiBaseUrl = "http://localhost:6969";

document.addEventListener("alpine:init", () => {
  Alpine.data("scedit", () => ({
    edit: false,
    toggle() {
      this.edit = !this.edit;
    },
  }));

  Alpine.data("formHandler", () => {
    const tagsElement = document.getElementById("tags-data");
    const existingTags = tagsElement
      ? JSON.parse(tagsElement.dataset.tags)
      : [];

    const emptyField = () => ({
      index: Date.now(),
      key: "",
      type: "string",
      is_repeated: false,
      is_required: false,
      inner_fields: [],
    });

    return {
      existingTags: existingTags,
      selected_tag: "Create new tag",
      method: "GET",
      path: "",
      description: "",
      path_params: "",
      body: [emptyField()],
      response: [emptyField()],

      shouldShowTagInput() {
        return !this.existingTags.includes(this.selected_tag);
      },

      initMapField(field) {
        if (!field.inner_fields) {
          field.inner_fields = [emptyField()];
        } else if (field.inner_fields.length === 0) {
          field.inner_fields.push(emptyField());
        }
      },

      addField(form) {
        this[form].push(emptyField());
      },

      clearForm() {
        this.existingTags = existingTags;
        this.selected_tag = "Create new tag";
        this.method = "GET";
        this.path = "";
        this.description = "";
        this.path_params = "";
        this.body = [emptyField()];
        this.response = [emptyField()];
      },

      addNestedField(parentField) {
        if (!parentField.inner_fields) {
          parentField.inner_fields = [];
        }
        parentField.inner_fields.push(emptyField());
      },

      removeField(index, form) {
        this[form].splice(index, 1);
      },

      submitForm() {
        const project_id = new URL(window.location.href).pathname.split("/")[2];
        const payload = {
          tag: this.selected_tag,
          method: this.method,
          path: this.path,
          path_params: this.path_params,
          description: this.description,
          body: transformFields(this.body),
          response: transformFields(this.response),
          project_id,
        };

        console.log("Form data:", payload);

        fetch(`${apiBaseUrl}/api/project/${project_id}/contract`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(payload),
        }).catch((e) => showToast(e));
      },
    };
  });
});
