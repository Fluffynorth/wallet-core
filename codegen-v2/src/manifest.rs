use core::panic;

use crate::{
    grammar::{GHeaderFileItem, GMarker, GType, GTypeCategory},
    CHeaderDirectory,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    BadImport,
    BadObject,
    BadProperty,
    BadType,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeInfo {
    pub variant: TypeVariant,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub is_pointer: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeVariant {
    Void,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Struct(String),
    Enum(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub imports: Vec<ImportInfo>,
    pub objects: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    // Expressed as directories plus the final file.
    // E.g. `to/some/file.h` ~= ["to", "some", "file.h"]
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumInfo {
    pub name: String,
    pub is_public: bool,
    pub variants: Vec<(String, Option<usize>)>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructInfo {
    pub name: String,
    pub is_public: bool,
    pub fields: Vec<(String, TypeInfo)>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyInfo {
    pub name: String,
    pub is_public: bool,
    pub is_static: bool,
    pub return_type: TypeInfo,
    pub comments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    pub name: String,
    pub is_public: bool,
    pub is_static: bool,
    pub params: Vec<ParamInfo>,
    pub return_type: TypeInfo,
    pub comments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    pub ty: TypeInfo,
}

// NOTE: This function is temporary
pub fn process_c_header_dir(dir: &CHeaderDirectory) {
    for (path, items) in &dir.map {
        println!("### {:?}", path);

        let mut found_struct = None;
        let mut found_enum = None;

        for item in items {
            if let GHeaderFileItem::StructDecl(decl) = item {
                let x = StructInfo::from_g_type(decl).unwrap();

                if found_struct.is_some() {
                    panic!("Found two structs in a row");
                }

                found_struct = Some(x.clone());

                let x = serde_json::to_string_pretty(&x).unwrap();
                println!("STRUCT: {}", x);
            }

            if let GHeaderFileItem::EnumDecl(decl) = item {
                let x = EnumInfo::from_g_type(decl).unwrap();

                if found_enum.is_some() {
                    panic!("Found two enums in a row");
                }

                found_enum = Some(x.clone());

                let x = serde_json::to_string_pretty(&x).unwrap();
                println!("ENUM: {}", x);
            }

            if let GHeaderFileItem::FunctionDecl(decl) = item {
                println!("MATCHED {:?}", decl.name);

                if decl.name.0.contains("CreateWith") || decl.name.0.contains("Delete") {
                    found_struct = None;
                    found_enum = None;
                    continue;
                }

                let object_name = path
                    .to_str()
                    .unwrap()
                    .split("/")
                    .last()
                    .unwrap()
                    .strip_suffix(".h")
                    .unwrap();

                dbg!(object_name);

                if decl.markers.0.contains(&GMarker::TwExportMethod)
                    || decl.markers.0.contains(&GMarker::TwExportStaticMethod)
                {
                    let x = MethodInfo::from_g_type(&Some(object_name.to_string()), decl).unwrap();
                    let x = serde_json::to_string_pretty(&x).unwrap();
                    println!("METHOD: {}", x);
                }
            }

            found_struct = None;
            found_enum = None;
        }
    }
}

pub fn extract_custom(ty: &GType) -> Option<String> {
    match ty {
        GType::Mutable(cat) | GType::Const(cat) | GType::Extern(cat) => {
            if let GTypeCategory::Unrecognized(keyword) = cat {
                Some(keyword.0.clone())
            } else {
                None
            }
        }
    }
}

#[test]
#[ignore]
fn test_manifest() {
    let path = std::path::Path::new("../include/");
    let dir = crate::parse(&path).unwrap();
    process_c_header_dir(&dir);
}
