// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Attribute(Boxed<ffi::PangoAttribute>);

    match fn {
        copy => |ptr| ffi::pango_attribute_copy(ptr),
        free => |ptr| ffi::pango_attribute_destroy(ptr),
        type_ => || ffi::pango_attribute_get_type(),
    }
}

#[cfg(not(any(feature = "v1_44", feature = "dox")))]
glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Attribute(Boxed<ffi::PangoAttribute>);

    match fn {
        copy => |ptr| ffi::pango_attribute_copy(ptr),
        free => |ptr| ffi::pango_attribute_destroy(ptr),
    }
}

impl Attribute {
    #[doc(alias = "pango_attribute_equal")]
    fn equal(&self, attr2: &Attribute) -> bool {
        unsafe {
            from_glib(ffi::pango_attribute_equal(
                self.to_glib_none().0,
                attr2.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Attribute {}

unsafe impl Send for Attribute {}
unsafe impl Sync for Attribute {}
