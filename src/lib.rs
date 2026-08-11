use ::xmpkit as wrapped_xmpkit;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::collections::HashMap;
use wrapped_xmpkit::core::metadata::XmpProperty as wrapped_XmpProperty;
use wrapped_xmpkit::core::namespace::NamespaceMap as wrapped_NamespaceMap;
use wrapped_xmpkit::{
    XmpDateTime as wrapped_XmpDateTime, XmpFile as wrapped_XmpFile, XmpMeta as wrapped_XmpMeta,
    XmpOptions as wrapped_XmpOptions, XmpValue as wrapped_XmpValue,
};

pyo3::create_exception!(xmpkit, XmpError, PyException);

#[pymodule]
mod xmpkit {
    use super::*;

    // XmpError

    #[pymodule_export]
    use XmpError;

    // XmpValue

    #[pyclass(from_py_object)]
    #[derive(Clone)]
    enum XmpValue {
        String(String),
        Integer(i64),
        Boolean(bool),
        DateTime(String),
        Array(Vec<XmpValue>),
        Structure(HashMap<String, XmpValue>),
    }

    impl XmpValue {
        fn wrap(wrapped: wrapped_XmpValue) -> Self {
            match wrapped {
                wrapped_XmpValue::String(v) => Self::String(v),
                wrapped_XmpValue::Integer(v) => Self::Integer(v),
                wrapped_XmpValue::Boolean(v) => Self::Boolean(v),
                wrapped_XmpValue::DateTime(v) => Self::DateTime(v),
                wrapped_XmpValue::Array(v) => {
                    let mut vec = Vec::with_capacity(v.len());
                    for el in v {
                        vec.push(Self::wrap(el));
                    }
                    Self::Array(vec)
                }
                wrapped_XmpValue::Structure(v) => {
                    let mut map = HashMap::with_capacity(v.len());
                    for (key, value) in v {
                        map.insert(key, Self::wrap(value));
                    }
                    Self::Structure(map)
                }
            }
        }

        fn unwrap(self) -> wrapped_XmpValue {
            match self {
                Self::String(v) => wrapped_XmpValue::String(v),
                Self::Integer(v) => wrapped_XmpValue::Integer(v),
                Self::Boolean(v) => wrapped_XmpValue::Boolean(v),
                Self::DateTime(v) => wrapped_XmpValue::DateTime(v),
                Self::Array(v) => {
                    let mut vec = Vec::with_capacity(v.len());
                    for el in v {
                        vec.push(Self::unwrap(el));
                    }
                    wrapped_XmpValue::Array(vec)
                }
                Self::Structure(v) => {
                    let mut map = HashMap::with_capacity(v.len());
                    for (key, value) in v {
                        map.insert(key, Self::unwrap(value));
                    }
                    wrapped_XmpValue::Structure(map)
                }
            }
        }
    }

    #[pymethods]
    impl XmpValue {
        fn as_str(&self) -> Option<&str> {
            match self {
                Self::String(s) => Some(s),
                Self::DateTime(s) => Some(s),
                _ => None,
            }
        }

        fn as_int(&self) -> Option<i64> {
            match self {
                Self::String(s) => s.parse::<i64>().ok(),
                Self::Integer(i) => Some(*i),
                _ => None,
            }
        }

        fn as_bool(&self) -> Option<bool> {
            match self {
                Self::String(s) => match s.as_str() {
                    "True" => Some(true),
                    "False" => Some(false),
                    _ => None,
                },
                Self::Boolean(b) => Some(*b),
                _ => None,
            }
        }

        fn as_array(&self) -> Option<Vec<XmpValue>> {
            match self {
                Self::Array(a) => Some(a.to_vec()),
                _ => None,
            }
        }

        fn as_structure(&self) -> Option<HashMap<String, XmpValue>> {
            match self {
                Self::Structure(s) => Some(s.clone()),
                _ => None,
            }
        }
    }

    // XmpDateTime

    #[pyclass(from_py_object)]
    #[derive(Clone)]
    struct XmpDateTime {
        wrapped: wrapped_XmpDateTime,
    }

    impl XmpDateTime {
        fn wrap(wrapped: wrapped_XmpDateTime) -> Self {
            Self { wrapped }
        }

        fn unwrap(self) -> wrapped_XmpDateTime {
            self.wrapped
        }
    }

    #[pymethods]
    impl XmpDateTime {
        #[new]
        #[pyo3(signature = (
            year=0,
            month=0,
            day=0,
            hour=0,
            minute=0,
            second=0,
            nanosecond=0,
            has_date=false,
            has_time=false,
            has_timezone=false,
            tz_sign=0,
            tz_hour=0,
            tz_minute=0,
        ))]
        fn new(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            has_date: bool,
            has_time: bool,
            has_timezone: bool,
            tz_sign: i8,
            tz_hour: u8,
            tz_minute: u8,
        ) -> Self {
            Self::wrap(wrapped_XmpDateTime {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nanosecond,
                has_date,
                has_time,
                has_timezone,
                tz_sign,
                tz_hour,
                tz_minute,
            })
        }

        #[classmethod]
        fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
            match wrapped_XmpDateTime::parse(s) {
                Ok(obj) => Ok(Self { wrapped: obj }),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn format(&self) -> String {
            self.wrapped.format()
        }

        fn validate(&self) -> PyResult<()> {
            match self.wrapped.validate() {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        // Fields

        #[getter]
        fn year(&self) -> i32 {
            self.wrapped.year
        }
        #[setter]
        fn set_year(&mut self, value: i32) {
            self.wrapped.year = value;
        }

        #[getter]
        fn month(&self) -> u8 {
            self.wrapped.month
        }
        #[setter]
        fn set_month(&mut self, value: u8) {
            self.wrapped.month = value;
        }

        #[getter]
        fn day(&self) -> u8 {
            self.wrapped.day
        }
        #[setter]
        fn set_day(&mut self, value: u8) {
            self.wrapped.day = value;
        }

        #[getter]
        fn hour(&self) -> u8 {
            self.wrapped.hour
        }
        #[setter]
        fn set_hour(&mut self, value: u8) {
            self.wrapped.hour = value;
        }

        #[getter]
        fn minute(&self) -> u8 {
            self.wrapped.minute
        }
        #[setter]
        fn set_minute(&mut self, value: u8) {
            self.wrapped.minute = value;
        }

        #[getter]
        fn second(&self) -> u8 {
            self.wrapped.second
        }
        #[setter]
        fn set_second(&mut self, value: u8) {
            self.wrapped.second = value;
        }

        #[getter]
        fn nanosecond(&self) -> u32 {
            self.wrapped.nanosecond
        }
        #[setter]
        fn set_nanosecond(&mut self, value: u32) {
            self.wrapped.nanosecond = value;
        }

        #[getter]
        fn has_date(&self) -> bool {
            self.wrapped.has_date
        }
        #[setter]
        fn set_has_date(&mut self, value: bool) {
            self.wrapped.has_date = value;
        }

        #[getter]
        fn has_time(&self) -> bool {
            self.wrapped.has_time
        }
        #[setter]
        fn set_has_time(&mut self, value: bool) {
            self.wrapped.has_time = value;
        }

        #[getter]
        fn has_timezone(&self) -> bool {
            self.wrapped.has_timezone
        }
        #[setter]
        fn set_has_timezone(&mut self, value: bool) {
            self.wrapped.has_timezone = value;
        }

        #[getter]
        fn tz_sign(&self) -> i8 {
            self.wrapped.tz_sign
        }
        #[setter]
        fn set_tz_sign(&mut self, value: i8) {
            self.wrapped.tz_sign = value;
        }

        #[getter]
        fn tz_hour(&self) -> u8 {
            self.wrapped.tz_hour
        }
        #[setter]
        fn set_tz_hour(&mut self, value: u8) {
            self.wrapped.tz_hour = value;
        }

        #[getter]
        fn tz_minute(&self) -> u8 {
            self.wrapped.tz_minute
        }
        #[setter]
        fn set_tz_minute(&mut self, value: u8) {
            self.wrapped.tz_minute = value;
        }
    }

    // XmpProperty

    #[pyclass]
    struct XmpProperty {
        #[pyo3(get)]
        namespace_uri: String,
        #[pyo3(get)]
        name: String,
        #[pyo3(get)]
        value: XmpValue,
    }

    impl XmpProperty {
        fn wrap(wrapped: wrapped_XmpProperty) -> Self {
            Self {
                namespace_uri: wrapped.namespace_uri,
                name: wrapped.name,
                value: XmpValue::wrap(wrapped.value),
            }
        }

        fn unwrap(self) -> wrapped_XmpProperty {
            wrapped_XmpProperty {
                namespace_uri: self.namespace_uri,
                name: self.name,
                value: self.value.unwrap(),
            }
        }
    }

    #[pymethods]
    impl XmpProperty {
        #[new]
        fn new(namespace_uri: String, name: String, value: XmpValue) -> Self {
            Self {
                namespace_uri,
                name,
                value,
            }
        }
    }

    // XmpMeta

    #[pyclass(from_py_object)]
    #[derive(Clone)]
    struct XmpMeta {
        wrapped: wrapped_XmpMeta,
    }

    impl XmpMeta {
        fn wrap(wrapped: wrapped_XmpMeta) -> Self {
            Self { wrapped }
        }

        fn unwrap(self) -> wrapped_XmpMeta {
            self.wrapped
        }
    }

    #[pymethods]
    impl XmpMeta {
        // Initialization

        #[new]
        fn new() -> Self {
            Self::wrap(wrapped_XmpMeta::new())
        }

        #[classmethod]
        fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
            match wrapped_XmpMeta::parse(s) {
                Ok(obj) => Ok(Self { wrapped: obj }),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        // Properties manipulation

        fn all_properties(&self) -> Vec<XmpProperty> {
            let props = self.wrapped.all_properties();
            let mut out = Vec::with_capacity(props.len());
            for prop in props {
                out.push(XmpProperty::wrap(prop));
            }
            out
        }

        fn has_property(&self, namespace: &str, path: &str) -> bool {
            self.wrapped.has_property(namespace, path)
        }

        fn get_property(&self, namespace: &str, path: &str) -> Option<XmpValue> {
            match self.wrapped.get_property(namespace, path) {
                None => None,
                Some(v) => Some(XmpValue::wrap(v)),
            }
        }

        fn set_property(&mut self, namespace: &str, path: &str, value: XmpValue) -> PyResult<()> {
            match self.wrapped.set_property(namespace, path, value.unwrap()) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn delete_property(&mut self, namespace: &str, path: &str) -> PyResult<()> {
            match self.wrapped.delete_property(namespace, path) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        // About URI

        #[getter]
        fn about_uri(&self) -> Option<&str> {
            self.wrapped.about_uri()
        }

        #[setter]
        fn set_about_uri(&mut self, uri: &str) {
            self.wrapped.set_about_uri(uri);
        }

        // Serialization

        fn serialize(&self) -> PyResult<String> {
            match self.wrapped.serialize() {
                Ok(s) => Ok(s),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn serialize_packet(&self) -> PyResult<String> {
            match self.wrapped.serialize_packet() {
                Ok(s) => Ok(s),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn serialize_packet_with_padding(&self, target_length: usize) -> PyResult<String> {
            match self.wrapped.serialize_packet_with_padding(target_length) {
                Ok(s) => Ok(s),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        // Arrays

        fn get_array_item(&self, namespace: &str, path: &str, index: usize) -> Option<XmpValue> {
            match self.wrapped.get_array_item(namespace, path, index) {
                None => None,
                Some(item) => Some(XmpValue::wrap(item)),
            }
        }

        fn get_array_size(&self, namespace: &str, path: &str) -> Option<usize> {
            self.wrapped.get_array_size(namespace, path)
        }

        fn append_array_item(
            &mut self,
            namespace: &str,
            path: &str,
            value: XmpValue,
        ) -> PyResult<()> {
            match self
                .wrapped
                .append_array_item(namespace, path, value.unwrap())
            {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn insert_array_item(
            &mut self,
            namespace: &str,
            path: &str,
            index: usize,
            value: XmpValue,
        ) -> PyResult<()> {
            match self
                .wrapped
                .insert_array_item(namespace, path, index, value.unwrap())
            {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn delete_array_item(&mut self, namespace: &str, path: &str, index: usize) -> PyResult<()> {
            match self.wrapped.delete_array_item(namespace, path, index) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        // Structures

        fn get_struct_field(
            &self,
            namespace: &str,
            struct_path: &str,
            field_name: &str,
        ) -> Option<XmpValue> {
            match self
                .wrapped
                .get_struct_field(namespace, struct_path, field_name)
            {
                None => None,
                Some(field) => Some(XmpValue::wrap(field)),
            }
        }

        fn set_struct_field(
            &mut self,
            namespace: &str,
            struct_path: &str,
            field_name: &str,
            value: XmpValue,
        ) -> PyResult<()> {
            match self
                .wrapped
                .set_struct_field(namespace, struct_path, field_name, value.unwrap())
            {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn delete_struct_field(
            &mut self,
            namespace: &str,
            struct_path: &str,
            field_name: &str,
        ) -> PyResult<()> {
            match self
                .wrapped
                .delete_struct_field(namespace, struct_path, field_name)
            {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        // Localized text

        fn set_localized_text(
            &mut self,
            namespace: &str,
            property: &str,
            _generic_lang: &str,
            specific_lang: &str,
            value: &str,
        ) -> PyResult<()> {
            match self.wrapped.set_localized_text(
                namespace,
                property,
                _generic_lang,
                specific_lang,
                value,
            ) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn get_localized_text(
            &self,
            namespace: &str,
            property: &str,
            generic_lang: &str,
            specific_lang: &str,
        ) -> Option<(String, String)> {
            self.wrapped
                .get_localized_text(namespace, property, generic_lang, specific_lang)
        }

        // Date/time

        fn set_date_time(&mut self, namespace: &str, path: &str, dt: XmpDateTime) -> PyResult<()> {
            match self.wrapped.set_date_time(namespace, path, &dt.unwrap()) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn get_date_time(&self, namespace: &str, path: &str) -> Option<XmpDateTime> {
            match self.wrapped.get_date_time(namespace, path) {
                None => None,
                Some(v) => Some(XmpDateTime::wrap(v)),
            }
        }
    }

    // XmpOptions

    #[pyclass(from_py_object)]
    #[derive(Clone)]
    struct XmpOptions {
        #[pyo3(get, set)]
        for_update: bool,
        #[pyo3(get, set)]
        only_xmp: bool,
        #[pyo3(get, set)]
        force_given_handler: bool,
        #[pyo3(get, set)]
        strict: bool,
        #[pyo3(get, set)]
        use_smart_handler: bool,
        #[pyo3(get, set)]
        use_packet_scanning: bool,
        #[pyo3(get, set)]
        limited_scanning: bool,
    }

    impl XmpOptions {
        fn wrap(wrapped: wrapped_XmpOptions) -> Self {
            Self {
                for_update: wrapped.for_update,
                only_xmp: wrapped.only_xmp,
                force_given_handler: wrapped.force_given_handler,
                strict: wrapped.strict,
                use_smart_handler: wrapped.use_smart_handler,
                use_packet_scanning: wrapped.use_packet_scanning,
                limited_scanning: wrapped.limited_scanning,
            }
        }

        fn unwrap(self) -> wrapped_XmpOptions {
            wrapped_XmpOptions {
                for_update: self.for_update,
                only_xmp: self.only_xmp,
                force_given_handler: self.force_given_handler,
                strict: self.strict,
                use_smart_handler: self.use_smart_handler,
                use_packet_scanning: self.use_packet_scanning,
                limited_scanning: self.limited_scanning,
            }
        }
    }

    #[pymethods]
    impl XmpOptions {
        #[new]
        #[pyo3(signature = (
            for_update=false,
            only_xmp=false,
            force_given_handler=false,
            strict=false,
            use_smart_handler=false,
            use_packet_scanning=false,
            limited_scanning=false,
        ))]
        fn new(
            for_update: bool,
            only_xmp: bool,
            force_given_handler: bool,
            strict: bool,
            use_smart_handler: bool,
            use_packet_scanning: bool,
            limited_scanning: bool,
        ) -> Self {
            Self {
                for_update,
                only_xmp,
                force_given_handler,
                strict,
                use_smart_handler,
                use_packet_scanning,
                limited_scanning,
            }
        }

        fn with_for_read<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.for_update = false;
            slf
        }

        fn with_for_update<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.for_update = true;
            slf
        }

        fn with_only_xmp<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.only_xmp = true;
            slf
        }

        fn with_force_given_handler<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.force_given_handler = true;
            slf
        }

        fn with_strict<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.strict = true;
            slf
        }

        fn with_use_smart_handler<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.use_smart_handler = true;
            slf
        }

        fn with_use_packet_scanning<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.use_packet_scanning = true;
            slf
        }

        fn with_limited_scanning<'a>(mut slf: PyRefMut<'a, Self>) -> PyRefMut<'a, Self> {
            slf.limited_scanning = true;
            slf
        }
    }

    // Namespace

    #[pyclass]
    struct NamespaceMap {
        wrapped: wrapped_NamespaceMap,
    }

    impl NamespaceMap {
        fn wrap(wrapped: wrapped_NamespaceMap) -> Self {
            Self { wrapped }
        }

        fn unwrap(self) -> wrapped_NamespaceMap {
            self.wrapped
        }
    }

    #[pymethods]
    impl NamespaceMap {
        #[new]
        fn new() -> Self {
            Self::wrap(wrapped_NamespaceMap::new())
        }

        fn register(&mut self, uri: &str, prefix: &str) -> PyResult<()> {
            match self.wrapped.register(uri, prefix) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn get_prefix(&self, uri: &str) -> Option<&str> {
            self.wrapped.get_prefix(uri)
        }

        fn get_uri(&self, prefix: &str) -> Option<&str> {
            self.wrapped.get_uri(prefix)
        }

        fn has_uri(&self, uri: &str) -> bool {
            self.wrapped.has_uri(uri)
        }

        fn has_prefix(&self, prefix: &str) -> bool {
            self.wrapped.has_prefix(prefix)
        }

        fn get_all_namespaces(&self) -> Vec<(String, String)> {
            self.wrapped.get_all_namespaces()
        }
    }

    #[pyfunction]
    fn register_namespace(uri: &str, prefix: &str) -> PyResult<()> {
        match wrapped_xmpkit::register_namespace(uri, prefix) {
            Ok(v) => Ok(v),
            Err(error) => Err(XmpError::new_err(error.to_string())),
        }
    }

    #[pyfunction]
    fn is_namespace_registered(uri: &str) -> bool {
        wrapped_xmpkit::is_namespace_registered(uri)
    }

    #[pyfunction]
    fn get_global_namespace_prefix(uri: &str) -> Option<String> {
        wrapped_xmpkit::get_global_namespace_prefix(uri)
    }

    #[pyfunction]
    fn get_global_namespace_uri(prefix: &str) -> Option<String> {
        wrapped_xmpkit::get_global_namespace_uri(prefix)
    }

    #[pyfunction]
    fn get_all_registered_namespaces() -> Vec<(String, String)> {
        wrapped_xmpkit::get_all_registered_namespaces()
    }

    #[pyfunction]
    fn get_builtin_namespace_uris() -> Vec<String> {
        wrapped_xmpkit::get_builtin_namespace_uris()
    }

    // Namespace constants

    #[pymodule]
    mod ns {
        use super::*;

        #[pymodule_export]
        const XMP: &str = wrapped_xmpkit::ns::XMP;
        #[pymodule_export]
        const DC: &str = wrapped_xmpkit::ns::DC;
        #[pymodule_export]
        const EXIF: &str = wrapped_xmpkit::ns::EXIF;
        #[pymodule_export]
        const EXIF_AUX: &str = wrapped_xmpkit::ns::EXIF_AUX;
        #[pymodule_export]
        const EXIF_EX: &str = wrapped_xmpkit::ns::EXIF_EX;
        #[pymodule_export]
        const IPTC_CORE: &str = wrapped_xmpkit::ns::IPTC_CORE;
        #[pymodule_export]
        const IPTC_EXT: &str = wrapped_xmpkit::ns::IPTC_EXT;
        #[pymodule_export]
        const PHOTOSHOP: &str = wrapped_xmpkit::ns::PHOTOSHOP;
        #[pymodule_export]
        const CAMERA_RAW: &str = wrapped_xmpkit::ns::CAMERA_RAW;
        #[pymodule_export]
        const XMP_RIGHTS: &str = wrapped_xmpkit::ns::XMP_RIGHTS;
        #[pymodule_export]
        const XMP_MM: &str = wrapped_xmpkit::ns::XMP_MM;
        #[pymodule_export]
        const XMP_BJ: &str = wrapped_xmpkit::ns::XMP_BJ;
        #[pymodule_export]
        const TIFF: &str = wrapped_xmpkit::ns::TIFF;
        #[pymodule_export]
        const PDF: &str = wrapped_xmpkit::ns::PDF;
        #[pymodule_export]
        const PDFX: &str = wrapped_xmpkit::ns::PDFX;
        #[pymodule_export]
        const PDFA: &str = wrapped_xmpkit::ns::PDFA;
        #[pymodule_export]
        const XMP_DM: &str = wrapped_xmpkit::ns::XMP_DM;
        #[pymodule_export]
        const XMP_PAGED: &str = wrapped_xmpkit::ns::XMP_PAGED;
        #[pymodule_export]
        const XMP_GRAPHICS: &str = wrapped_xmpkit::ns::XMP_GRAPHICS;
        #[pymodule_export]
        const XMP_IMAGE: &str = wrapped_xmpkit::ns::XMP_IMAGE;
        #[pymodule_export]
        const RDF: &str = wrapped_xmpkit::ns::RDF;
        #[pymodule_export]
        const XML: &str = wrapped_xmpkit::ns::XML;
        #[pymodule_export]
        const XMP_PREFIX: &str = wrapped_xmpkit::ns::XMP_PREFIX;
        #[pymodule_export]
        const DC_PREFIX: &str = wrapped_xmpkit::ns::DC_PREFIX;
        #[pymodule_export]
        const EXIF_PREFIX: &str = wrapped_xmpkit::ns::EXIF_PREFIX;
        #[pymodule_export]
        const RDF_PREFIX: &str = wrapped_xmpkit::ns::RDF_PREFIX;
        #[pymodule_export]
        const XML_PREFIX: &str = wrapped_xmpkit::ns::XML_PREFIX;
        #[pymodule_export]
        const EXIF_AUX_PREFIX: &str = wrapped_xmpkit::ns::EXIF_AUX_PREFIX;
        #[pymodule_export]
        const EXIF_EX_PREFIX: &str = wrapped_xmpkit::ns::EXIF_EX_PREFIX;
        #[pymodule_export]
        const IPTC_CORE_PREFIX: &str = wrapped_xmpkit::ns::IPTC_CORE_PREFIX;
        #[pymodule_export]
        const IPTC_EXT_PREFIX: &str = wrapped_xmpkit::ns::IPTC_EXT_PREFIX;
        #[pymodule_export]
        const PHOTOSHOP_PREFIX: &str = wrapped_xmpkit::ns::PHOTOSHOP_PREFIX;
        #[pymodule_export]
        const CAMERA_RAW_PREFIX: &str = wrapped_xmpkit::ns::CAMERA_RAW_PREFIX;
        #[pymodule_export]
        const XMP_RIGHTS_PREFIX: &str = wrapped_xmpkit::ns::XMP_RIGHTS_PREFIX;
        #[pymodule_export]
        const XMP_MM_PREFIX: &str = wrapped_xmpkit::ns::XMP_MM_PREFIX;
        #[pymodule_export]
        const XMP_BJ_PREFIX: &str = wrapped_xmpkit::ns::XMP_BJ_PREFIX;
        #[pymodule_export]
        const TIFF_PREFIX: &str = wrapped_xmpkit::ns::TIFF_PREFIX;
        #[pymodule_export]
        const PDF_PREFIX: &str = wrapped_xmpkit::ns::PDF_PREFIX;
        #[pymodule_export]
        const PDFX_PREFIX: &str = wrapped_xmpkit::ns::PDFX_PREFIX;
        #[pymodule_export]
        const PDFA_PREFIX: &str = wrapped_xmpkit::ns::PDFA_PREFIX;
        #[pymodule_export]
        const XMP_DM_PREFIX: &str = wrapped_xmpkit::ns::XMP_DM_PREFIX;
        #[pymodule_export]
        const XMP_PAGED_PREFIX: &str = wrapped_xmpkit::ns::XMP_PAGED_PREFIX;
        #[pymodule_export]
        const XMP_GRAPHICS_PREFIX: &str = wrapped_xmpkit::ns::XMP_GRAPHICS_PREFIX;
        #[pymodule_export]
        const XMP_IMAGE_PREFIX: &str = wrapped_xmpkit::ns::XMP_IMAGE_PREFIX;
    }

    // XmpFile

    #[pyclass]
    struct XmpFile {
        wrapped: wrapped_XmpFile,
    }

    impl XmpFile {
        fn wrap(wrapped: wrapped_XmpFile) -> Self {
            Self { wrapped }
        }

        fn unwrap(self) -> wrapped_XmpFile {
            self.wrapped
        }
    }

    #[pymethods]
    impl XmpFile {
        #[new]
        fn new() -> Self {
            Self::wrap(wrapped_XmpFile::new())
        }

        fn open_with(&mut self, path: &str, options: XmpOptions) -> PyResult<()> {
            match self.wrapped.open_with(path, options.unwrap()) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        #[staticmethod]
        fn scan_for_xmp_packet(file_data: &[u8]) -> PyResult<Option<XmpMeta>> {
            match wrapped_XmpFile::scan_for_xmp_packet(file_data) {
                Ok(v) => Ok(match v {
                    None => None,
                    Some(meta) => Some(XmpMeta::wrap(meta)),
                }),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn open(&mut self, path: &str) -> PyResult<()> {
            match self.wrapped.open(path) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn from_bytes(&mut self, data: &[u8]) -> PyResult<()> {
            match self.wrapped.from_bytes(data) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn from_bytes_with(&mut self, data: &[u8], options: XmpOptions) -> PyResult<()> {
            match self.wrapped.from_bytes_with(data, options.unwrap()) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn get_xmp(&self) -> Option<XmpMeta> {
            match self.wrapped.get_xmp() {
                None => None,
                Some(meta) => Some(XmpMeta::wrap(meta.clone())),
            }
        }

        fn put_xmp(&mut self, meta: XmpMeta) {
            self.wrapped.put_xmp(meta.unwrap());
        }

        fn close(&mut self) {
            self.wrapped.close();
        }

        fn try_close(&mut self) -> PyResult<()> {
            match self.wrapped.try_close() {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn save(&self, path: &str) -> PyResult<()> {
            match self.wrapped.save(path) {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }

        fn write_to_bytes(&self) -> PyResult<Vec<u8>> {
            match self.wrapped.write_to_bytes() {
                Ok(v) => Ok(v),
                Err(error) => Err(XmpError::new_err(error.to_string())),
            }
        }
    }
}
