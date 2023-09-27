use ext_php_rs::convert::FromZval;

use crate::PostType;

use super::Params;

impl<'a> FromZval<'a> for Params<'a> {
    const TYPE: ext_php_rs::flags::DataType = ext_php_rs::flags::DataType::Array;

    fn from_zval(zval: &'a ext_php_rs::types::Zval) -> Option<Self> {
        if !zval.is_array() {
            return None;
        }

        let mut params = Self::new();

        if let Some(array) = zval.array() {
            /* Author params */
            if let Some(auth_id) = array.get("author") {
                params.author = auth_id.long().map(|l| l as u64);
            }

            if let Some(auth_name) = array.get("author_name") {
                params.author_name = auth_name.str();
            }

            if let Some(auth_ids) = array.get("author__in").map(|v| v.array()).flatten() {
                params.author__in = auth_ids.try_into().ok();
            }

            if let Some(auth_ids) = array.get("author__not_in").map(|v| v.array()).flatten() {
                params.author__not_in = auth_ids.try_into().ok();
            }

            /* Post Type */
            // PHP Version allows for array or string, accounts for both possibilies
            if let Some(post_types) = array.get("post_type").map(|r| r.array()).flatten() {
                let p_types: Vec<PostType> = post_types
                    .iter()
                    .filter_map(|p_type| p_type.2.str())
                    .map(|p_type| PostType::from(p_type))
                    .collect();
                params.post_type = Some(p_types)
            } else if let Some(post_type) = array.get("post_type").map(|v| v.str()).flatten() {
                params.post_type = Some(vec![PostType::from(post_type)]);
            }

            /* Posts Per Page */
            if let Some(per_page) = array
                .get("posts_per_page")
                .map(|v| v.long())
                .flatten()
                .map(|n| n as u64)
            {
                params.posts_per_page = Some(per_page);
            }
        }

        Some(params)
    }
}
