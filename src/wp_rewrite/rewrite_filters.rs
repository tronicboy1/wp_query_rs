use super::WpRewrite;

pub enum RewriteFilters {
    PostRewriteRules,
}

/// Create a filter struct for a given parent struct and property.
///
macro_rules! filter_cache_struct {
    ($parent: ident, $parent_prop: ident, $name: ident { $($filter_name: ident: $callback_type: path,)* }) => {
        pub struct $name {
            $($filter_name: Vec<Box<dyn $callback_type>>,)*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($filter_name: Vec::new(),)*
                }
            }
            $(pub fn $filter_name<F>(&mut self, f: F) where F: $callback_type + 'static {
                self.$filter_name.push(Box::new(f));
            })*
        }

        impl $parent {
            pub fn add_filter(&mut self) -> &mut $name {
                &mut self.$parent_prop
            }
        }
    };

}

filter_cache_struct!(
    WpRewrite,
    hooks,
    RewriteFilterCache {
        post_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        date_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        category_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        post_format_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        post_tag_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        search_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        comments_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        author_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        page_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        root_rewrite_rules: Fn(Vec<String>) -> Vec<String>,
        rewrite_rules_array: Fn(Vec<String>) -> Vec<String>,
        generate_rewrite_rules: Fn(WpRewrite) -> WpRewrite,
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_filters() {
        let mut filter_cache = RewriteFilterCache::new();

        filter_cache.post_rewrite_rules(|rules| rules);
        filter_cache.generate_rewrite_rules(|mut rewrite| {
            rewrite.author_base.push_str("Hello World");
            rewrite
        });

        assert_eq!(filter_cache.post_rewrite_rules.len(), 1);
        assert_eq!(filter_cache.generate_rewrite_rules.len(), 1);
    }
}
