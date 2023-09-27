use ext_php_rs::{
    boxed::ZBox,
    convert::{FromZval, IntoZval},
    ffi::_zend_object,
    flags::DataType,
    types::{ZendObject, Zval},
};

use super::WpPost;

macro_rules! apply_to_all_fields {
    ($obj: ident, $method: ident, $self: expr) => {
        $obj.$method("ID", $self.ID)?;
        $obj.$method("post_status", $self.post_status)?;
        $obj.$method("post_author", $self.post_author)?;
        $obj.$method("post_date", $self.post_date.to_string())?;
        $obj.$method("post_date_gmt", $self.post_date_gmt.to_string())?;
        $obj.$method("post_content", $self.post_content)?;
        $obj.$method("post_title", $self.post_title)?;
        $obj.$method("post_excerpt", $self.post_excerpt)?;
        $obj.$method("comment_status", $self.comment_status)?;
        $obj.$method("ping_status", $self.ping_status)?;
        $obj.$method("post_password", $self.post_password)?;
        $obj.$method("post_name", $self.post_name)?;
        $obj.$method("to_ping", $self.to_ping)?;
        $obj.$method("pinged", $self.pinged)?;
        $obj.$method("post_modified", $self.post_modified.to_string())?;
        $obj.$method("post_modified_gmt", $self.post_modified_gmt.to_string())?;
        $obj.$method("post_content_filtered", $self.post_content_filtered)?;
        $obj.$method("post_parent", $self.post_parent)?;
        $obj.$method("guid", $self.guid)?;
        $obj.$method("menu_order", $self.menu_order)?;
        $obj.$method("post_type", $self.post_type)?;
        $obj.$method("post_mime_type", $self.post_mime_type)?;
        $obj.$method("comment_count", $self.comment_count)?;
    };
}

impl WpPost {
    fn build_zobj(self) -> ext_php_rs::error::Result<ZBox<_zend_object>> {
        let mut zobj = ZendObject::new_stdclass();

        apply_to_all_fields!(zobj, set_property, self);

        Ok(zobj)
    }
}

impl IntoZval for WpPost {
    const TYPE: ext_php_rs::flags::DataType = DataType::Object(Some("RS_WP_Post"));

    fn into_zval(self, persistent: bool) -> ext_php_rs::error::Result<ext_php_rs::types::Zval> {
        let zobj = self.build_zobj()?;

        zobj.into_zval(persistent)
    }

    fn set_zval(self, zv: &mut Zval, _persistent: bool) -> ext_php_rs::error::Result<()> {
        let mut zobj = self.build_zobj()?;

        zv.set_object(&mut zobj);

        Ok(())
    }
}

impl<'a> FromZval<'a> for WpPost {
    const TYPE: ext_php_rs::flags::DataType = DataType::Object(Some("RS_WP_Post"));

    // Do not implement as not used, must satisfy ext-php-rs traits
    fn from_zval(zval: &'a Zval) -> Option<Self> {
        if let Some(_array) = zval.array() {}

        if let Some(_obj) = zval.object() {}

        None
    }
}
