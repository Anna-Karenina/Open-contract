pub fn parse_path_template(path: &str) -> (String, Vec<String>) {
    let segments: Vec<&str> = path.split('/').collect();
    let mut path_parts = Vec::new();
    let mut query_params = Vec::new();

    for segment in segments {
        if segment.starts_with('{') && segment.ends_with('}') {
            // Извлекаем параметр (убираем { и })
            let param = &segment[1..segment.len() - 1];
            query_params.push(param.to_string());
            // Добавляем в путь имя параметра в CamelCase (например, ProjectId)
            let camel_case_param = param
                .split('_')
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<String>();
            path_parts.push(camel_case_param);
        } else {
            // Преобразуем обычный сегмент в CamelCase
            let camel_case_segment = segment
                .split('_')
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<String>();
            path_parts.push(camel_case_segment);
        }
    }

    // Собираем итоговый путь (например, "ApiProjectProjectIdList")
    let path = path_parts.join("");

    (path, query_params)
}
