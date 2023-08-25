use std::collections::HashMap;

use crate::{
    sql::{SqlOrder, SqlSearchOperators},
    wp_post::post_status::PostStatus,
    DateQuery, MetaQuery, MetaRelation, Params,
};

use super::{
    orderby::WpOrderBy,
    tax_query::{TaxQuery, TaxRelation},
};

/// Builds query params by chaining option callbacks
///
/// # Examples
///
/// ```
/// use wp_query_rs::{ParamBuilder, PostStatus};
///
/// let params = ParamBuilder::new().page(2)
///     .post_status(PostStatus::AutoDraft)
///     .posts_per_page(20);
/// ```
///
/// # Panics
///
/// Will panic if you provide faulty datetime values
///
/// ```rust,ignore
/// use wp_query_rs::ParamBuilder;
///
/// let params = ParamBuilder::new()
///     .hour(24); // InvalidHour
/// ```
pub type ParamBuilder = Params;

#[allow(non_snake_case)]
impl ParamBuilder {
    /// use author id
    pub fn author(mut self, author_id: u64) -> Self {
        self.author = Some(author_id);

        self
    }

    /// use ‘user_nicename‘ – NOT name.
    pub fn author_name(mut self, s: &str) -> Self {
        self.author_name = Some(s.to_string());

        self
    }

    /// use author id
    pub fn author__in(mut self, author_id: u64) -> Self {
        let mut authors = self.author__in.unwrap_or(Vec::new());
        authors.push(author_id);

        self.author__in = Some(authors);

        self
    }

    /// use author id
    pub fn author__not_in(mut self, author_id: u64) -> Self {
        let mut authors = self.author__not_in.unwrap_or(Vec::new());
        authors.push(author_id);

        self.author__not_in = Some(authors);

        self
    }

    pub fn cat(mut self, cat_id: u64) -> Self {
        self.cat = Some(cat_id);

        self
    }

    pub fn category_name(mut self, s: &str) -> Self {
        self.category_name = Some(s.to_string());

        self
    }

    fn category__and(mut self, cat_id: u64) -> Self {
        let mut ids = self.category__and.unwrap_or(Vec::new());
        ids.push(cat_id);

        self.category__and = Some(ids);

        self
    }

    pub fn category__in(mut self, cat_id: u64) -> Self {
        let mut ids = self.category__in.unwrap_or(Vec::new());
        ids.push(cat_id);

        self.category__in = Some(ids);

        self
    }

    pub fn category__not_in(mut self, cat_id: u64) -> Self {
        let mut ids = self.category__not_in.unwrap_or(Vec::new());
        ids.push(cat_id);

        self.category__not_in = Some(ids);

        self
    }

    pub fn tag(mut self, slug: &str) -> Self {
        self.tag = Some(slug.to_string());

        self
    }

    pub fn tag_id(mut self, tag_id: u64) -> Self {
        self.tag_id = Some(tag_id);

        self
    }

    fn tag__and(self) -> Self {
        self
    }

    fn tag__in(self) -> Self {
        self
    }

    fn tag__not_in(self) -> Self {
        self
    }

    fn tag_slug__and(self) -> Self {
        self
    }

    fn tag_slug__in(self) -> Self {
        self
    }

    pub fn tax_query(mut self, query: TaxQuery, relation: Option<TaxRelation>) -> Self {
        let mut tax_q = self.tax_query.unwrap_or(HashMap::new());

        if let Some(rel) = relation {
            let qs_for_relation = tax_q.entry(rel).or_insert(vec![]);
            qs_for_relation.push(query);

            self.tax_query = Some(tax_q);
        } else {
            self.tax_query = Some(TaxQuery::new_single_tax_map(query));
        }

        self
    }

    /// Search keyword
    pub fn s(mut self, s: &str) -> Self {
        self.s = Some(s.to_string());

        self
    }

    /// use post id
    pub fn p(mut self, id: u64) -> Self {
        self.p = Some(id);

        self
    }

    /// use post slug
    pub fn name(mut self, slug: &str) -> Self {
        self.name = Some(slug.to_string());

        self
    }

    fn page_id(self) -> Self {
        self
    }

    fn pagename(self) -> Self {
        self
    }

    /// use page id to return only child pages. Set to 0 to return only top-level entries.
    pub fn post_parent(mut self, id: u64) -> Self {
        self.post_parent = Some(id);

        self
    }

    /// use post ids. Specify posts whose parent is in an array
    pub fn post_parent__in(mut self, id: u64) -> Self {
        let mut ids = self.post_parent__in.unwrap_or(Vec::new());

        ids.push(id);

        self.post_parent__in = Some(ids);

        self
    }

    /// use post ids. Specify posts whose parent is not in an array
    pub fn post_parent__not_in(mut self, id: u64) -> Self {
        let mut ids = self.post_parent__not_in.unwrap_or(Vec::new());

        ids.push(id);

        self.post_parent__not_in = Some(ids);

        self
    }

    /// use post ids. Specify posts to retrieve.
    pub fn post__in(mut self, id: u64) -> Self {
        let mut ids = self.post__in.unwrap_or(Vec::new());

        ids.push(id);

        self.post__in = Some(ids);

        self
    }

    /// use post ids. Specify post NOT to retrieve.
    pub fn post__not_in(mut self, id: u64) -> Self {
        let mut ids = self.post__not_in.unwrap_or(Vec::new());

        ids.push(id);

        self.post__not_in = Some(ids);

        self
    }

    pub fn post_name__in(mut self, s: &str) -> Self {
        let mut names = self.post_name__in.unwrap_or(Vec::new());

        names.push(s.to_string());

        self.post_name__in = Some(names);

        self
    }

    fn post_password(self) -> Self {
        self
    }

    /// use post types. Retrieves posts by post types, default value is ‘post‘.
    pub fn post_type(mut self, post_type: &str) -> Self {
        let mut types = self.post_type.unwrap_or(Vec::new());
        dbg!(&types);

        types.push(post_type.to_string());

        self.post_type = Some(types);

        self
    }

    /// Queries all post types. Will be overwritten if there is another call to post_type after this.
    pub fn post_type_all(mut self) -> Self {
        self.post_type = Some(Vec::new());

        self
    }

    pub fn post_status(mut self, status: PostStatus) -> Self {
        self.post_status = Some(status);

        self
    }

    fn comment_count(mut self, count: u64) -> Self {
        self.comment_count = Some(count);

        self
    }

    pub fn posts_per_page(mut self, n: u64) -> Self {
        self.posts_per_page = Some(n);

        self
    }

    /// Starts from page 1
    pub fn page(mut self, n: u64) -> Self {
        self.page = Some(n - 1);

        self
    }

    fn ignore_sticky_posts(self) -> Self {
        self
    }

    pub fn order(mut self, o: SqlOrder) -> Self {
        self.order = Some(o);

        self
    }

    /// Sort retrieved posts by parameter.
    pub fn orderby(mut self, ob: WpOrderBy) -> Self {
        self.orderby = Some(ob);

        self
    }

    /// 4 digit year (e.g. 2011).
    pub fn year(mut self, y: u16) -> Self {
        if y > 9999 {
            panic!("InvalidYear");
        }

        self.year = Some(y);

        self
    }

    /// Month number (from 1 to 12).
    pub fn monthnum(mut self, m: u8) -> Self {
        if m > 12 || m < 1 {
            panic!("InvalidMonth");
        }

        self.monthnum = Some(m);

        self
    }

    ///  Week of the year (from 0 to 53). Uses MySQL WEEK command. The mode is dependent on the “start_of_week” option.
    pub fn w(mut self, w: u8) -> Self {
        if w > 53 {
            panic!("InalidWeekNo");
        }

        self.w = Some(w);

        self
    }

    /// Day of the month (from 1 to 31).
    pub fn day(mut self, d: u8) -> Self {
        if d > 31 || d < 1 {
            panic!("InvalidDay");
        }

        self.day = Some(d);

        self
    }

    /// Hour (from 0 to 23).
    pub fn hour(mut self, h: u8) -> Self {
        if h > 23 {
            panic!("InvalidHour");
        }

        self.hour = Some(h);

        self
    }

    /// Minute (from 0 to 60).
    pub fn minute(mut self, min: u8) -> Self {
        if min > 60 {
            panic!("InvalidMinutes");
        }

        self.minute = Some(min);

        self
    }

    /// Second (0 to 60).
    pub fn second(mut self, s: u8) -> Self {
        if s > 60 {
            panic!("InvalidSeconds");
        }

        self.second = Some(s);

        self
    }

    /// YearMonth (For e.g.: 201307).
    fn m(mut self, m: u64) -> Self {
        if m > 999999 {
            panic!("InvalidYearMonth");
        }

        self.monthnum = None;
        self.year = None;
        self.m = Some(m);

        self
    }

    pub fn date_query(mut self, query: DateQuery) -> Self {
        let mut queries = self.date_query.unwrap_or(Vec::new());

        queries.push(query);

        self.date_query = Some(queries);

        self
    }

    /// Custom field key.
    pub fn meta_key(mut self, key: &str) -> Self {
        if self.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        self.meta_key = Some(key.to_string());

        self
    }

    /// Custom field value.
    pub fn meta_value(mut self, val: &str) -> Self {
        if self.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        if self.meta_value_num.is_some() {
            self.meta_value_num = None;
        }

        self.meta_value = Some(val.to_string());

        self
    }

    /// Custom field value (number).
    pub fn meta_value_num(mut self, n: i64) -> Self {
        if self.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        if self.meta_value.is_some() {
            self.meta_value = None;
        }

        self.meta_value_num = Some(n);

        self
    }

    /// Operator to test the ‘meta_value‘
    pub fn meta_compare(mut self, compare: SqlSearchOperators) -> Self {
        self.meta_compare = Some(compare);

        self
    }

    pub fn meta_query(mut self, query: MetaQuery, relation: MetaRelation) -> Self {
        // Clear single meta
        self.meta_compare = None;
        self.meta_key = None;
        self.meta_value = None;
        self.meta_value_num = None;

        let mut meta_qs = self.meta_query.unwrap_or(HashMap::new());

        let queries_for_relation = meta_qs.entry(relation).or_insert(vec![]);

        queries_for_relation.push(query);

        self.meta_query = Some(meta_qs);

        self
    }

    fn post_mime_type(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_author() {
        let id = 1;
        let q = ParamBuilder::new().author(id);
        assert_eq!(id, q.author.unwrap());
    }

    #[test]
    fn can_add_author_in() {
        let id = 1;
        let q = ParamBuilder::new().author__in(id);
        assert_eq!(id, *q.author__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_author_not_in() {
        let id = 1;
        let q = ParamBuilder::new().author__not_in(id);
        assert_eq!(id, *q.author__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category() {
        let cat = 1;
        let q = ParamBuilder::new().cat(cat);
        assert_eq!(cat, q.cat.unwrap());
    }

    #[test]
    fn can_add_category_and() {
        let id = 1;
        let q = ParamBuilder::new().category__and(id);
        assert_eq!(id, *q.category__and.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category_in() {
        let id = 1;
        let q = ParamBuilder::new().category__in(id);
        assert_eq!(id, *q.category__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category_not_in() {
        let id = 1;
        let q = ParamBuilder::new().category__not_in(id);
        assert_eq!(id, *q.category__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_tag() {
        let tag = "Tag";
        let q = ParamBuilder::new().tag(tag);
        assert_eq!(tag, q.tag.unwrap());
    }

    #[test]
    fn can_add_single_tax() {
        let tax_name = String::from("custom_tax");
        let terms = vec![String::from("1")];
        let tax = TaxQuery::new(tax_name.clone(), terms.clone());
        let q = ParamBuilder::new().tax_query(tax, None);
        let stored = q.tax_query.unwrap();
        assert!(stored.get(&TaxRelation::Single).is_some());
        let stored = stored.get(&TaxRelation::Single).unwrap().first().unwrap();
        assert_eq!(stored.taxonomy, tax_name);
        assert_eq!(stored.terms, terms);
    }

    #[test]
    fn can_add_multiple_tax() {
        let tax_name = String::from("custom_tax");
        let tax_name_two = String::from("category");
        let tax_name_three = String::from("cust_2");
        let terms = vec![String::from("1")];
        let tax1 = TaxQuery::new(tax_name.clone(), terms.clone());
        let tax2 = TaxQuery::new(tax_name_two.clone(), terms.clone());
        let tax3 = TaxQuery::new(tax_name_three.clone(), terms.clone());
        let q = ParamBuilder::new()
            .tax_query(tax1, Some(TaxRelation::And))
            .tax_query(tax2, Some(TaxRelation::And))
            .tax_query(tax3, Some(TaxRelation::Or));

        let created = q.tax_query.unwrap();
        assert_eq!(created.len(), 2);
        assert_eq!(created.get(&TaxRelation::And).unwrap().len(), 2);
        assert_eq!(created.get(&TaxRelation::Or).unwrap().len(), 1);
    }

    #[test]
    fn can_add_post_params() {
        let q = ParamBuilder::new()
            .p(1)
            .post_parent(2)
            .post_status(PostStatus::Publish);
        assert_eq!(q.p.unwrap(), 1);
        assert_eq!(q.post_parent.unwrap(), 2);
        assert_eq!(q.post_status.unwrap(), PostStatus::Publish);
    }

    #[test]
    fn can_add_post_parent_in() {
        let id = 1;
        let q = ParamBuilder::new().post_parent__in(id);
        assert_eq!(id, *q.post_parent__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_parent_not_in() {
        let id = 1;
        let q = ParamBuilder::new().post_parent__not_in(id);
        assert_eq!(id, *q.post_parent__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_in() {
        let id = 1;
        let q = ParamBuilder::new().post__in(id);
        assert_eq!(id, *q.post__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_not_in() {
        let id = 1;
        let q = ParamBuilder::new().post__not_in(id);
        assert_eq!(id, *q.post__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_name_in() {
        let id = 1;
        let q = ParamBuilder::new()
            .post_name__in("malcolm-x")
            .post_name__in("mlk");
        let r = q.post_name__in.unwrap();
        assert_eq!(r.first().unwrap(), "malcolm-x");
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn can_add_post_type() {
        let q = ParamBuilder::new().post_type("page");
        assert_eq!(q.post_type.unwrap().first().unwrap(), "page");
    }

    #[test]
    fn can_add_multiple_post_types() {
        let q = ParamBuilder::new()
            .post_type("post")
            .post_type("page")
            .p(1)
            .post_parent(2)
            .post_status(PostStatus::Publish);
        assert_eq!(q.post_type.unwrap().len(), 2);
    }

    #[test]
    fn can_add_comment_params() {
        let q = ParamBuilder::new().comment_count(2);
        assert_eq!(q.comment_count.unwrap(), 2);
    }

    #[test]
    fn can_add_pagination_params() {
        let q = ParamBuilder::new().page(3).posts_per_page(20);
        assert_eq!(q.page.unwrap(), 2);
        assert_eq!(q.posts_per_page.unwrap(), 20);
    }

    #[test]
    fn can_add_orderby_params() {
        let q = ParamBuilder::new()
            .orderby(WpOrderBy::Author)
            .order(SqlOrder::Asc);
        assert_eq!(q.orderby.unwrap(), WpOrderBy::Author);
        assert_eq!(q.order.unwrap(), SqlOrder::Asc);
    }

    #[test]
    fn can_add_date_params() {
        let q = ParamBuilder::new()
            .year(2023)
            .monthnum(12)
            .monthnum(1)
            .w(53)
            .day(31)
            .hour(23)
            .minute(60)
            .second(60);
        assert_eq!(q.year.unwrap(), 2023);
        assert_eq!(q.monthnum.unwrap(), 1);
        assert_eq!(q.w.unwrap(), 53);
        assert_eq!(q.day.unwrap(), 31);
        assert_eq!(q.hour.unwrap(), 23);
        assert_eq!(q.minute.unwrap(), 60);
        assert_eq!(q.second.unwrap(), 60);
    }

    #[test]
    fn can_add_date_queries() {
        let dq1 = DateQuery::new().after(crate::DateQueryAfterBefore::new(2022, 2, 2));
        let dq2 = DateQuery::new();
        let q = ParamBuilder::new().date_query(dq1).date_query(dq2);
        let dq = q.date_query.unwrap();
        assert_eq!(dq.len(), 2);
        assert_eq!(dq.first().unwrap().after.as_ref().unwrap().day, 2);
    }

    #[test]
    fn m_clears_year_and_monthnum() {
        let q = ParamBuilder::new().year(2000).monthnum(7).m(202308);
        assert!(q.year.is_none());
        assert!(q.monthnum.is_none());
        assert_eq!(q.m.unwrap(), 202308);
    }

    #[test]
    fn can_set_single_meta() {
        let q = ParamBuilder::new()
            .meta_key("key1")
            .meta_value("a")
            .meta_compare(SqlSearchOperators::Like);
        assert_eq!(q.meta_key.unwrap(), "key1");
        assert_eq!(q.meta_value.unwrap(), "a");
        assert_eq!(q.meta_compare.unwrap(), SqlSearchOperators::Like);
    }

    #[test]
    fn can_set_multiple_meta() {
        let q = ParamBuilder::new()
            .meta_key("key1")
            .meta_query(
                MetaQuery {
                    key: String::from("key1"),
                    value: String::from("1"),
                    compare: SqlSearchOperators::Equals,
                },
                MetaRelation::And,
            )
            .meta_query(
                MetaQuery {
                    key: String::from("key2"),
                    value: String::from("2"),
                    compare: SqlSearchOperators::GreaterThan,
                },
                MetaRelation::And,
            );
        let queries = q.meta_query.unwrap();
        assert_eq!(queries.get(&MetaRelation::And).unwrap().len(), 2);
    }
}
