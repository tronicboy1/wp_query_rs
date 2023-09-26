use std::{collections::HashMap, fmt::Display};

use crate::{
    sql::{SqlOrder, SqlSearchOperators},
    wp_post::post_status::PostStatus,
    DateQuery, MetaQuery, MetaRelation, Params, PostType,
};

use super::{
    orderby::WpOrderBy,
    tax_query::{TaxQuery, TaxRelation},
    traits::{MetaQueryable, PostQueryable},
};

/// Builds query params by chaining option callbacks
///
/// # Examples
///
/// ```
/// use wp_query_rs::{ParamBuilder, PostStatus, PostQueryable};
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
pub struct ParamBuilder<'a>(Params<'a>);


/// Adds a value to an internal optional vector property for Params
macro_rules! add_to_vec {
    ($self: ident, $prop: ident, $v_key: ident) => {{
        let vec = $self.0.$prop.get_or_insert(Vec::new());
        vec.push($v_key);

        $self
    }};
}

#[allow(non_snake_case)]
impl<'a> ParamBuilder<'a> {
    pub fn new() -> Self {
        Self(Params::new())
    }

    /// use author id
    pub fn author(mut self, author_id: u64) -> Self {
        self.0.author = Some(author_id);

        self
    }

    /// use ‘user_nicename‘ – NOT name.
    pub fn author_name(mut self, s: &'a str) -> Self {
        self.0.author_name = Some(s);

        self
    }

    /// use author id
    pub fn author__in(mut self, author_id: u64) -> Self {
        add_to_vec!(self, author__in, author_id)
    }

    /// use author id
    pub fn author__not_in(mut self, author_id: u64) -> Self {
        add_to_vec!(self, author__not_in, author_id)
    }

    /// Searches by category ID
    pub fn cat(mut self, cat_id: u64) -> Self {
        add_to_vec!(self, term_and, cat_id)
    }

    pub fn category_name(mut self, s: &'a str) -> Self {
        add_to_vec!(self, term_slug_and, s)
    }

    pub fn category__and(mut self, cat_id: u64) -> Self {
        add_to_vec!(self, term_and, cat_id)
    }

    pub fn category__in(mut self, cat_id: u64) -> Self {
        add_to_vec!(self, term_in, cat_id)
    }

    pub fn category__not_in(mut self, cat_id: u64) -> Self {
        add_to_vec!(self, term_not_in, cat_id)
    }

    pub fn tag(mut self, slug: &'a str) -> Self {
        add_to_vec!(self, term_slug_and, slug)
    }

    pub fn tag_id(mut self, tag_id: u64) -> Self {
        add_to_vec!(self, term_and, tag_id)
    }

    pub fn tag__and(mut self, tag_id: u64) -> Self {
        add_to_vec!(self, term_and, tag_id)
    }

    pub fn tag__in(mut self, tag_id: u64) -> Self {
        add_to_vec!(self, term_in, tag_id)
    }

    pub fn tag__not_in(mut self, tag_id: u64) -> Self {
        add_to_vec!(self, term_not_in, tag_id)
    }

    pub fn tag_slug__and(mut self, tag_slug: &'a str) -> Self {
        add_to_vec!(self, term_slug_and, tag_slug)
    }

    pub fn tag_slug__in(mut self, tag_slug: &'a str) -> Self {
        add_to_vec!(self, term_slug_in, tag_slug)
    }

    pub fn tax_query(mut self, query: TaxQuery<'a>, relation: Option<TaxRelation>) -> Self {
        let mut tax_q = self.0.tax_query.unwrap_or(HashMap::new());

        if let Some(rel) = relation {
            let qs_for_relation = tax_q.entry(rel).or_insert(vec![]);
            qs_for_relation.push(query);

            self.0.tax_query = Some(tax_q);
        } else {
            self.0.tax_query = Some(TaxQuery::new_single_tax_map(query));
        }

        self
    }

    /// Search keyword
    pub fn s(mut self, s: &'a str) -> Self {
        self.0.s = Some(s);

        self
    }

    /// use post id
    pub fn p(mut self, id: u64) -> Self {
        self.0.p = Some(id);

        self
    }

    /// use post slug
    pub fn name(mut self, slug: &'a str) -> Self {
        self.0.name = Some(slug);

        self
    }

    fn page_id(self) -> Self {
        self
    }

    fn pagename(self) -> Self {
        self
    }

    fn comment_count(mut self, count: u64) -> Self {
        self.0.comment_count = Some(count);

        self
    }

    pub fn posts_per_page(mut self, n: u64) -> Self {
        self.0.posts_per_page = Some(n);

        self
    }

    /// Starts from page 1
    pub fn page(mut self, n: u64) -> Self {
        self.0.page = Some(n - 1);

        self
    }

    fn ignore_sticky_posts(self) -> Self {
        self
    }

    pub fn order(mut self, o: SqlOrder) -> Self {
        self.0.order = Some(o);

        self
    }

    /// Sort retrieved posts by parameter.
    pub fn orderby(mut self, ob: WpOrderBy) -> Self {
        self.0.orderby = Some(ob);

        self
    }

    /// 4 digit year (e.g. 2011).
    pub fn year(mut self, y: u16) -> Self {
        if y > 9999 {
            panic!("InvalidYear");
        }

        self.0.year = Some(y);

        self
    }

    /// Month number (from 1 to 12).
    pub fn monthnum(mut self, m: u8) -> Self {
        if m > 12 || m < 1 {
            panic!("InvalidMonth");
        }

        self.0.monthnum = Some(m);

        self
    }

    ///  Week of the year (from 0 to 53). Uses MySQL WEEK command. The mode is dependent on the “start_of_week” option.
    pub fn w(mut self, w: u8) -> Self {
        if w > 53 {
            panic!("InalidWeekNo");
        }

        self.0.w = Some(w);

        self
    }

    /// Day of the month (from 1 to 31).
    pub fn day(mut self, d: u8) -> Self {
        if d > 31 || d < 1 {
            panic!("InvalidDay");
        }

        self.0.day = Some(d);

        self
    }

    /// Hour (from 0 to 23).
    pub fn hour(mut self, h: u8) -> Self {
        if h > 23 {
            panic!("InvalidHour");
        }

        self.0.hour = Some(h);

        self
    }

    /// Minute (from 0 to 60).
    pub fn minute(mut self, min: u8) -> Self {
        if min > 60 {
            panic!("InvalidMinutes");
        }

        self.0.minute = Some(min);

        self
    }

    /// Second (0 to 60).
    pub fn second(mut self, s: u8) -> Self {
        if s > 60 {
            panic!("InvalidSeconds");
        }

        self.0.second = Some(s);

        self
    }

    /// YearMonth (For e.g.: 201307).
    fn m(mut self, m: u64) -> Self {
        if m > 999999 {
            panic!("InvalidYearMonth");
        }

        self.0.monthnum = None;
        self.0.year = None;
        self.0.m = Some(m);

        self
    }

    pub fn date_query(mut self, query: DateQuery) -> Self {
        let mut queries = self.0.date_query.unwrap_or(Vec::new());

        queries.push(query);

        self.0.date_query = Some(queries);

        self
    }

    fn post_mime_type(self) -> Self {
        self
    }
}

#[allow(non_snake_case)]
impl<'a> PostQueryable<'a> for ParamBuilder<'a> {
    /// use page id to return only child pages. Set to 0 to return only top-level entries.
    fn post_parent(mut self, id: u64) -> Self {
        self.0.post_parent = Some(id);

        self
    }

    /// use post ids. Specify posts whose parent is in an array
    fn post_parent__in(mut self, id: u64) -> Self {
        add_to_vec!(self, post_parent__in, id)
    }

    /// use post ids. Specify posts whose parent is not in an array
    fn post_parent__not_in(mut self, id: u64) -> Self {
        add_to_vec!(self, post_parent__not_in, id)
    }

    /// use post ids. Specify posts to retrieve.
    fn post__in(mut self, id: u64) -> Self {
        add_to_vec!(self, post__in, id)
    }

    /// use post ids. Specify post NOT to retrieve.
    fn post__not_in(mut self, id: u64) -> Self {
        add_to_vec!(self, post__not_in, id)
    }

    fn post_name__in(mut self, s: &'a str) -> Self {
        add_to_vec!(self, post_name__in, s)
    }

    /// use post types. Retrieves posts by post types, default value is ‘post‘.
    fn post_type(mut self, post_type: PostType<'a>) -> Self {
        add_to_vec!(self, post_type, post_type)
    }

    /// Queries all post types. Will be overwritten if there is another call to post_type after this.
    fn post_type_all(mut self) -> Self {
        self.0.post_type = Some(Vec::new());

        self
    }

    fn post_status(mut self, status: PostStatus) -> Self {
        self.0.post_status = Some(status);

        self
    }
}

impl<'a> MetaQueryable<'a> for ParamBuilder<'a> {
    /// Custom field key.
    fn meta_key(mut self, key: &'a str) -> Self {
        if self.0.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        self.0.meta_key = Some(key);

        self
    }

    /// Custom field value.
    fn meta_value(mut self, val: impl Display) -> Self {
        if self.0.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        if self.0.meta_value_num.is_some() {
            self.0.meta_value_num = None;
        }

        self.0.meta_value = Some(val.to_string());

        self
    }

    /// Custom field value (number).
    fn meta_value_num(mut self, n: i64) -> Self {
        if self.0.meta_query.is_some() {
            panic!("CannotAddSingleMetaKeyQueryWhenMetaQueryIsSet");
        }

        if self.0.meta_value.is_some() {
            self.0.meta_value = None;
        }

        self.0.meta_value_num = Some(n);

        self
    }

    /// Operator to test the ‘meta_value‘
    fn meta_compare(mut self, compare: SqlSearchOperators) -> Self {
        self.0.meta_compare = Some(compare);

        self
    }

    fn meta_query(mut self, query: MetaQuery, relation: MetaRelation) -> Self {
        // Clear single meta
        self.0.meta_compare = None;
        self.0.meta_key = None;
        self.0.meta_value = None;
        self.0.meta_value_num = None;

        let meta_qs = self.0.meta_query.get_or_insert(HashMap::new());

        let queries_for_relation = meta_qs.entry(relation).or_insert(vec![]);

        queries_for_relation.push(query);

        self
    }
}

impl<'a> Into<Params<'a>> for ParamBuilder<'a> {
    fn into(self) -> Params<'a> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_author() {
        let id = 1;
        let q = ParamBuilder::new().author(id);
        assert_eq!(id, q.0.author.unwrap());
    }

    #[test]
    fn can_add_author_in() {
        let id = 1;
        let q = ParamBuilder::new().author__in(id);
        assert_eq!(&id, q.0.author__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_author_not_in() {
        let id = 1;
        let q = ParamBuilder::new().author__not_in(id);
        assert_eq!(id, *q.0.author__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category() {
        let cat = 1;
        let q = ParamBuilder::new().cat(cat);
        assert_eq!(cat, q.0.term_and.unwrap()[0]);
    }

    #[test]
    fn can_add_category_and() {
        let id = 1;
        let q = ParamBuilder::new().category__and(id);
        assert_eq!(id, *q.0.term_and.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category_in() {
        let id = 1;
        let q = ParamBuilder::new().category__in(id);
        assert_eq!(id, *q.0.term_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_category_not_in() {
        let id = 1;
        let q = ParamBuilder::new().category__not_in(id);
        assert_eq!(id, *q.0.term_not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_tag() {
        let tag = "Tag";
        let q = ParamBuilder::new().tag(tag);
        assert_eq!(tag, q.0.term_slug_and.unwrap()[0]);
    }

    #[test]
    fn can_add_tag_and() {
        let q = ParamBuilder::new().tag__and(42).tag__and(27);

        let tag_and = q.0.term_and.unwrap();
        assert_eq!(tag_and[0], 42);
        assert_eq!(tag_and[1], 27);
    }

    #[test]
    fn can_add_tag_in() {
        let q = ParamBuilder::new().tag__in(42).tag__in(27);

        let tag_in = q.0.term_in.unwrap();
        assert_eq!(tag_in[0], 42);
        assert_eq!(tag_in[1], 27);
    }

    #[test]
    fn can_add_tag_not_in() {
        let q = ParamBuilder::new().tag__not_in(42).tag__not_in(27);

        let tag_in = q.0.term_not_in.unwrap();
        assert_eq!(tag_in[0], 42);
        assert_eq!(tag_in[1], 27);
    }

    #[test]
    fn can_add_tag_slug_and() {
        let q = ParamBuilder::new()
            .tag_slug__and("russian")
            .tag_slug__and("food");

        let tag_slug_and = q.0.term_slug_and.unwrap();
        assert_eq!(tag_slug_and[0], String::from("russian"));
        assert_eq!(tag_slug_and[1], String::from("food"));
    }

    #[test]
    fn can_add_tag_slug_in() {
        let q = ParamBuilder::new()
            .tag_slug__in("russian")
            .tag_slug__in("food");

        let tag_slug_in = q.0.term_slug_in.unwrap();
        assert_eq!(tag_slug_in[0], String::from("russian"));
        assert_eq!(tag_slug_in[1], String::from("food"));
    }

    #[test]
    fn can_add_single_tax() {
        let terms = vec![String::from("1")];
        let tax = TaxQuery::new("custom_tax", terms.clone());
        let q = ParamBuilder::new().tax_query(tax, None);
        let stored = q.0.tax_query.unwrap();
        assert!(stored.get(&TaxRelation::Single).is_some());
        let stored = stored.get(&TaxRelation::Single).unwrap().first().unwrap();
        assert_eq!(stored.taxonomy, "custom_tax");
        assert_eq!(stored.terms, terms);
    }

    #[test]
    fn can_add_multiple_tax() {
        let tax_name = String::from("custom_tax");
        let tax_name_two = String::from("category");
        let tax_name_three = String::from("cust_2");
        let terms = vec![String::from("1")];
        let tax1 = TaxQuery::new("custom_tax", terms.clone());
        let tax2 = TaxQuery::new("category", terms.clone());
        let tax3 = TaxQuery::new("cust_2", terms.clone());
        let q = ParamBuilder::new()
            .tax_query(tax1, Some(TaxRelation::And))
            .tax_query(tax2, Some(TaxRelation::And))
            .tax_query(tax3, Some(TaxRelation::Or));

        let created = q.0.tax_query.unwrap();
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
        assert_eq!(q.0.p.unwrap(), 1);
        assert_eq!(q.0.post_parent.unwrap(), 2);
        assert_eq!(q.0.post_status.unwrap(), PostStatus::Publish);
    }

    #[test]
    fn can_add_post_parent_in() {
        let id = 1;
        let q = ParamBuilder::new().post_parent__in(id);
        assert_eq!(id, *q.0.post_parent__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_parent_not_in() {
        let id = 1;
        let q = ParamBuilder::new().post_parent__not_in(id);
        assert_eq!(id, *q.0.post_parent__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_in() {
        let id = 1;
        let q = ParamBuilder::new().post__in(id);
        assert_eq!(id, *q.0.post__in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_not_in() {
        let id = 1;
        let q = ParamBuilder::new().post__not_in(id);
        assert_eq!(id, *q.0.post__not_in.unwrap().first().unwrap());
    }

    #[test]
    fn can_add_post_name_in() {
        let id = 1;
        let q = ParamBuilder::new()
            .post_name__in("malcolm-x")
            .post_name__in("mlk");
        let r = q.0.post_name__in.unwrap();
        assert_eq!(r.first().unwrap(), &"malcolm-x");
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn can_add_post_type() {
        let q = ParamBuilder::new().post_type(PostType::Page);
        assert_eq!(q.0.post_type.unwrap().first().unwrap(), &PostType::Page);
    }

    #[test]
    fn can_add_multiple_post_types() {
        let q = ParamBuilder::new()
            .post_type(PostType::Post)
            .post_type(PostType::Page)
            .p(1)
            .post_parent(2)
            .post_status(PostStatus::Publish);
        assert_eq!(q.0.post_type.unwrap().len(), 2);
    }

    #[test]
    fn can_add_comment_params() {
        let q = ParamBuilder::new().comment_count(2);
        assert_eq!(q.0.comment_count.unwrap(), 2);
    }

    #[test]
    fn can_add_pagination_params() {
        let q = ParamBuilder::new().page(3).posts_per_page(20);
        assert_eq!(q.0.page.unwrap(), 2);
        assert_eq!(q.0.posts_per_page.unwrap(), 20);
    }

    #[test]
    fn can_add_orderby_params() {
        let q = ParamBuilder::new()
            .orderby(WpOrderBy::Author)
            .order(SqlOrder::Asc);
        assert_eq!(q.0.orderby.unwrap(), WpOrderBy::Author);
        assert_eq!(q.0.order.unwrap(), SqlOrder::Asc);
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
        assert_eq!(q.0.year.unwrap(), 2023);
        assert_eq!(q.0.monthnum.unwrap(), 1);
        assert_eq!(q.0.w.unwrap(), 53);
        assert_eq!(q.0.day.unwrap(), 31);
        assert_eq!(q.0.hour.unwrap(), 23);
        assert_eq!(q.0.minute.unwrap(), 60);
        assert_eq!(q.0.second.unwrap(), 60);
    }

    #[test]
    fn can_add_date_queries() {
        let dq1 = DateQuery::new().after(crate::DateQueryAfterBefore::new(2022, 2, 2));
        let dq2 = DateQuery::new();
        let q = ParamBuilder::new().date_query(dq1).date_query(dq2);
        let dq = q.0.date_query.unwrap();
        assert_eq!(dq.len(), 2);
        assert_eq!(dq.first().unwrap().after.as_ref().unwrap().day, 2);
    }

    #[test]
    fn m_clears_year_and_monthnum() {
        let q = ParamBuilder::new().year(2000).monthnum(7).m(202308);
        assert!(q.0.year.is_none());
        assert!(q.0.monthnum.is_none());
        assert_eq!(q.0.m.unwrap(), 202308);
    }

    #[test]
    fn can_set_single_meta() {
        let q = ParamBuilder::new()
            .meta_key("key1")
            .meta_value("a")
            .meta_compare(SqlSearchOperators::Like);
        assert_eq!(q.0.meta_key.unwrap(), "key1");
        assert_eq!(q.0.meta_value.unwrap(), "a");
        assert_eq!(q.0.meta_compare.unwrap(), SqlSearchOperators::Like);
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
        let queries = q.0.meta_query.unwrap();
        assert_eq!(queries.get(&MetaRelation::And).unwrap().len(), 2);
    }
}
