use std::collections::HashMap;

use crate::{
    query::{
        date_query::DateQuery,
        meta_query::{MetaQuery, MetaRelation},
        orderby::WpOrderBy,
        params::Params,
        tax_query::{TaxQuery, TaxRelation},
    },
    sql::{SqlOrder, SqlSearchOperators},
    wp_post::post_status::PostStatus,
};

pub struct ParamBuilder {
    pub query: Params,
}

#[allow(non_snake_case)]
impl ParamBuilder {
    pub fn params(self) -> Params {
        self.query
    }

    pub fn new() -> Self {
        Self {
            query: Params::default(),
        }
    }

    /**
     * use author id
     */
    pub fn author(mut self, author_id: u64) -> Self {
        self.query.author = Some(author_id);

        self
    }

    /**
     * use ‘user_nicename‘ – NOT name.
     */
    pub fn author_name(mut self, s: String) -> Self {
        self.query.author_name = Some(s);

        self
    }

    /**
     * use author id
     */
    pub fn author__in(mut self, author_id: u64) -> Self {
        let mut authors = self.query.author__in.unwrap_or(Vec::new());
        authors.push(author_id);

        self.query.author__in = Some(authors);

        self
    }

    /**
     * use author id
     */
    pub fn author__not_in(mut self, author_id: u64) -> Self {
        let mut authors = self.query.author__not_in.unwrap_or(Vec::new());
        authors.push(author_id);

        self.query.author__not_in = Some(authors);

        self
    }

    pub fn cat(mut self, cat_id: u64) -> Self {
        self.query.cat = Some(cat_id);

        self
    }

    pub fn category_name(mut self, s: String) -> Self {
        self.query.category_name = Some(s);

        self
    }

    fn category__and(mut self, cat_id: u64) -> Self {
        let mut ids = self.query.category__and.unwrap_or(Vec::new());
        ids.push(cat_id);

        self.query.category__and = Some(ids);

        self
    }

    pub fn category__in(mut self, cat_id: u64) -> Self {
        let mut ids = self.query.category__in.unwrap_or(Vec::new());
        ids.push(cat_id);

        self.query.category__in = Some(ids);

        self
    }

    pub fn category__not_in(mut self, cat_id: u64) -> Self {
        let mut ids = self.query.category__not_in.unwrap_or(Vec::new());
        ids.push(cat_id);

        self.query.category__not_in = Some(ids);

        self
    }

    pub fn tag(mut self, slug: String) -> Self {
        self.query.tag = Some(slug);

        self
    }

    pub fn tag_id(mut self, tag_id: u64) -> Self {
        self.query.tag_id = Some(tag_id);

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
        let mut tax_q = self.query.tax_query.unwrap_or(HashMap::new());

        if let Some(rel) = relation {
            let qs_for_relation = tax_q.entry(rel).or_insert(vec![]);
            qs_for_relation.push(query);

            self.query.tax_query = Some(tax_q);
        } else {
            self.query.tax_query = Some(TaxQuery::new_single_tax_map(query));
        }

        self
    }

    /**
     * Search keyword
     */
    pub fn s(mut self, s: String) -> Self {
        self.query.s = Some(s);

        self
    }

    /**
     * use post id
     */
    pub fn p(mut self, id: u64) -> Self {
        self.query.p = Some(id);

        self
    }

    /**
     * use post slug
     */
    pub fn name(mut self, slug: String) -> Self {
        self.query.name = Some(slug);

        self
    }

    fn page_id(self) -> Self {
        self
    }

    fn pagename(self) -> Self {
        self
    }

    /**
     * use page id to return only child pages. Set to 0 to return only top-level entries.
     */
    pub fn post_parent(mut self, id: u64) -> Self {
        self.query.post_parent = Some(id);

        self
    }

    /**
     * use post ids. Specify posts whose parent is in an array
     */
    pub fn post_parent__in(mut self, id: u64) -> Self {
        let mut ids = self.query.post_parent__in.unwrap_or(Vec::new());

        ids.push(id);

        self.query.post_parent__in = Some(ids);

        self
    }

    /**
     * use post ids. Specify posts whose parent is not in an array
     */
    pub fn post_parent__not_in(mut self, id: u64) -> Self {
        let mut ids = self.query.post_parent__not_in.unwrap_or(Vec::new());

        ids.push(id);

        self.query.post_parent__not_in = Some(ids);

        self
    }

    /**
     * use post ids. Specify posts to retrieve.
     */
    pub fn post__in(mut self, id: u64) -> Self {
        let mut ids = self.query.post__in.unwrap_or(Vec::new());

        ids.push(id);

        self.query.post__in = Some(ids);

        self
    }

    /**
     * use post ids. Specify post NOT to retrieve.
     */
    pub fn post__not_in(mut self, id: u64) -> Self {
        let mut ids = self.query.post__not_in.unwrap_or(Vec::new());

        ids.push(id);

        self.query.post__not_in = Some(ids);

        self
    }

    pub fn post_name__in(mut self, s: String) -> Self {
        let mut names = self.query.post_name__in.unwrap_or(Vec::new());

        names.push(s);

        self.query.post_name__in = Some(names);

        self
    }

    fn post_password(self) -> Self {
        self
    }

    pub fn post_type(mut self, post_types: Vec<String>) -> Self {
        self.query.post_type = Some(post_types);

        self
    }

    pub fn post_status(mut self, status: PostStatus) -> Self {
        self.query.post_status = Some(status);

        self
    }

    fn comment_count(mut self, count: u64) -> Self {
        self.query.comment_count = Some(count);

        self
    }

    pub fn posts_per_page(mut self, n: u64) -> Self {
        self.query.posts_per_page = Some(n);

        self
    }

    /**
     * Starts from page 1
     */
    pub fn page(mut self, n: u64) -> Self {
        self.query.page = Some(n - 1);

        self
    }

    fn ignore_sticky_posts(self) -> Self {
        self
    }

    pub fn order(mut self, o: SqlOrder) -> Self {
        self.query.order = Some(o);

        self
    }

    /**
     * Sort retrieved posts by parameter.
     */
    pub fn orderby(mut self, ob: WpOrderBy) -> Self {
        self.query.orderby = Some(ob);

        self
    }

    /**
     * 4 digit year (e.g. 2011).
     */
    pub fn year(mut self, y: u16) -> Self {
        if y > 9999 {
            panic!("InvalidYear");
        }

        self.query.year = Some(y);

        self
    }

    /**
     * Month number (from 1 to 12).
     */
    pub fn monthnum(mut self, m: u8) -> Self {
        if m > 12 || m < 1 {
            panic!("InvalidMonth");
        }

        self.query.monthnum = Some(m);

        self
    }

    /**
     *  Week of the year (from 0 to 53). Uses MySQL WEEK command. The mode is dependent on the “start_of_week” option.
     */
    pub fn w(mut self, w: u8) -> Self {
        if w > 53 {
            panic!("InalidWeekNo");
        }

        self.query.w = Some(w);

        self
    }

    /**
     * Day of the month (from 1 to 31).
     */
    pub fn day(mut self, d: u8) -> Self {
        if d > 31 || d < 1 {
            panic!("InvalidDay");
        }

        self.query.day = Some(d);

        self
    }

    /**
     * Hour (from 0 to 23).
     */
    pub fn hour(mut self, h: u8) -> Self {
        if h > 23 {
            panic!("InvalidHour");
        }

        self.query.hour = Some(h);

        self
    }

    /**
     * Minute (from 0 to 60).
     */
    pub fn minute(mut self, min: u8) -> Self {
        if min > 60 {
            panic!("InvalidMinutes");
        }

        self.query.minute = Some(min);

        self
    }

    /**
     * Second (0 to 60).
     */
    pub fn second(mut self, s: u8) -> Self {
        if s > 60 {
            panic!("InvalidSeconds");
        }

        self.query.second = Some(s);

        self
    }

    /**
     * YearMonth (For e.g.: 201307).
     */
    fn m(mut self, m: u64) -> Self {
        if m > 999999 {
            panic!("InvalidYearMonth");
        }

        self.query.monthnum = None;
        self.query.year = None;
        self.query.m = Some(m);

        self
    }

    pub fn date_query(mut self, query: DateQuery) -> Self {
        let mut queries = self.query.date_query.unwrap_or(Vec::new());

        queries.push(query);

        self.query.date_query = Some(queries);

        self
    }

    /**
     *  Custom field key.
     */
    pub fn meta_key(mut self, key: String) -> Self {
        if self.query.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        self.query.meta_key = Some(key);

        self
    }

    /**
     * Custom field value.
     */
    pub fn meta_value(mut self, val: String) -> Self {
        if self.query.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        if self.query.meta_value_num.is_some() {
            self.query.meta_value_num = None;
        }

        self.query.meta_value = Some(val);

        self
    }

    /**
     * Custom field value (number).
     */
    pub fn meta_value_num(mut self, n: i64) -> Self {
        if self.query.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        if self.query.meta_value.is_some() {
            self.query.meta_value = None;
        }

        self.query.meta_value_num = Some(n);

        self
    }

    /**
     * Operator to test the ‘meta_value‘
     */
    pub fn meta_compare(mut self, compare: SqlSearchOperators) -> Self {
        self.query.meta_compare = Some(compare);

        self
    }

    pub fn meta_query(mut self, query: MetaQuery, relation: MetaRelation) -> Self {
        // Clear single meta
        self.query.meta_compare = None;
        self.query.meta_key = None;
        self.query.meta_value = None;
        self.query.meta_value_num = None;

        let mut meta_qs = self.query.meta_query.unwrap_or(HashMap::new());

        let queries_for_relation = meta_qs.entry(relation).or_insert(vec![]);

        queries_for_relation.push(query);

        self.query.meta_query = Some(meta_qs);

        self
    }

    fn post_mime_type(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::query::date_query::DateQueryAfterBefore;

    use super::*;

    #[test]
    fn can_add_author() {
        let id = 1;
        let q = ParamBuilder::new().author(id);
        assert_eq!(id, q.query.author.unwrap());
    }

    #[test]
    fn can_add_author_in() {
        let id = 1;
        let q = ParamBuilder::new().author__in(id);
        assert_eq!(id, *q.query.author__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_author_not_in() {
        let id = 1;
        let q = ParamBuilder::new().author__not_in(id);
        assert_eq!(id, *q.query.author__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category() {
        let cat = 1;
        let q = ParamBuilder::new().cat(cat);
        assert_eq!(cat, q.query.cat.unwrap());
    }

    #[test]
    fn can_add_category_and() {
        let id = 1;
        let q = ParamBuilder::new().category__and(id);
        assert_eq!(id, *q.query.category__and.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category_in() {
        let id = 1;
        let q = ParamBuilder::new().category__in(id);
        assert_eq!(id, *q.query.category__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category_not_in() {
        let id = 1;
        let q = ParamBuilder::new().category__not_in(id);
        assert_eq!(id, *q.query.category__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_tag() {
        let tag = String::from("Tag");
        let q = ParamBuilder::new().tag(tag.clone());
        assert_eq!(tag, q.query.tag.unwrap());
    }

    #[test]
    fn can_add_single_tax() {
        let tax_name = String::from("custom_tax");
        let terms = vec![String::from("1")];
        let tax = TaxQuery::new(tax_name.clone(), terms.clone());
        let q = ParamBuilder::new().tax_query(tax, None);
        let stored = q.query.tax_query.unwrap();
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

        let created = q.query.tax_query.unwrap();
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
        assert_eq!(q.query.p.unwrap(), 1);
        assert_eq!(q.query.post_parent.unwrap(), 2);
        assert_eq!(q.query.post_status.unwrap(), PostStatus::Publish);
    }

    #[test]
    fn can_add_post_parent_in() {
        let id = 1;
        let q = ParamBuilder::new().post_parent__in(id);
        assert_eq!(id, *q.query.post_parent__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_parent_not_in() {
        let id = 1;
        let q = ParamBuilder::new().post_parent__not_in(id);
        assert_eq!(id, *q.query.post_parent__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_in() {
        let id = 1;
        let q = ParamBuilder::new().post__in(id);
        assert_eq!(id, *q.query.post__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_not_in() {
        let id = 1;
        let q = ParamBuilder::new().post__not_in(id);
        assert_eq!(id, *q.query.post__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_name_in() {
        let id = 1;
        let q = ParamBuilder::new()
            .post_name__in(String::from("malcolm-x"))
            .post_name__in(String::from("mlk"));
        let r = q.query.post_name__in.unwrap();
        assert_eq!(r.first().unwrap(), "malcolm-x");
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn can_add_post_type() {
        let q = ParamBuilder::new().post_type(vec![String::from("page")]);
        assert_eq!(q.query.post_type.unwrap().first().unwrap(), "page");
    }

    #[test]
    fn can_add_multiple_post_types() {
        let q = ParamBuilder::new()
            .post_type(vec![String::from("post"), String::from("page")])
            .p(1)
            .post_parent(2)
            .post_status(PostStatus::Publish);
        assert_eq!(q.query.post_type.unwrap().len(), 2);
    }

    #[test]
    fn default_post_type() {
        let q = ParamBuilder::new();
        assert_eq!(q.query.post_type.unwrap().first().unwrap(), "post");
    }

    #[test]
    fn can_add_comment_params() {
        let q = ParamBuilder::new().comment_count(2);
        assert_eq!(q.query.comment_count.unwrap(), 2);
    }

    #[test]
    fn can_add_pagination_params() {
        let q = ParamBuilder::new().page(3).posts_per_page(20);
        assert_eq!(q.query.page.unwrap(), 2);
        assert_eq!(q.query.posts_per_page.unwrap(), 20);
    }

    #[test]
    fn can_add_orderby_params() {
        let q = ParamBuilder::new()
            .orderby(WpOrderBy::Author)
            .order(SqlOrder::Asc);
        assert_eq!(q.query.orderby.unwrap(), WpOrderBy::Author);
        assert_eq!(q.query.order.unwrap(), SqlOrder::Asc);
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
        assert_eq!(q.query.year.unwrap(), 2023);
        assert_eq!(q.query.monthnum.unwrap(), 1);
        assert_eq!(q.query.w.unwrap(), 53);
        assert_eq!(q.query.day.unwrap(), 31);
        assert_eq!(q.query.hour.unwrap(), 23);
        assert_eq!(q.query.minute.unwrap(), 60);
        assert_eq!(q.query.second.unwrap(), 60);
    }

    #[test]
    fn can_add_date_queries() {
        let dq1 = DateQuery::new().after(DateQueryAfterBefore::new(2022, 2, 2));
        let dq2 = DateQuery::new();
        let q = ParamBuilder::new().date_query(dq1).date_query(dq2);
        let dq = q.query.date_query.unwrap();
        assert_eq!(dq.len(), 2);
        assert_eq!(dq.first().unwrap().after.as_ref().unwrap().day, 2);
    }

    #[test]
    fn m_clears_year_and_monthnum() {
        let q = ParamBuilder::new().year(2000).monthnum(7).m(202308);
        assert!(q.query.year.is_none());
        assert!(q.query.monthnum.is_none());
        assert_eq!(q.query.m.unwrap(), 202308);
    }

    #[test]
    fn can_set_single_meta() {
        let q = ParamBuilder::new()
            .meta_key(String::from("key1"))
            .meta_value(String::from("a"))
            .meta_compare(SqlSearchOperators::Like);
        assert_eq!(q.query.meta_key.unwrap(), "key1");
        assert_eq!(q.query.meta_value.unwrap(), "a");
        assert_eq!(q.query.meta_compare.unwrap(), SqlSearchOperators::Like);
    }

    #[test]
    fn can_set_multiple_meta() {
        let q = ParamBuilder::new()
            .meta_key(String::from("key1"))
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
        let queries = q.query.meta_query.unwrap();
        assert_eq!(queries.get(&MetaRelation::And).unwrap().len(), 2);
    }
}
