use ext_php_rs::convert::IntoZval;

use crate::PostStatus;

impl IntoZval for PostStatus {
    const TYPE: ext_php_rs::flags::DataType = ext_php_rs::flags::DataType::String;

    fn into_zval(self, persistent: bool) -> ext_php_rs::error::Result<ext_php_rs::types::Zval> {
        self.to_string().into_zval(persistent)
    }

    fn set_zval(
        self,
        zv: &mut ext_php_rs::types::Zval,
        persistent: bool,
    ) -> ext_php_rs::error::Result<()> {
        zv.set_string(&self.to_string(), persistent)
    }
}
