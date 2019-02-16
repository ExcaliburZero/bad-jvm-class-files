extern crate jvm_class_file_parser;

use std::env;
use std::fs::File;

use jvm_class_file_parser::{
    ClassAccess, ClassFile, ConstantPoolEntry
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];

    create_class_file(filepath);
}

fn create_class_file(filepath: &str) {
    let mut file = File::create(filepath).unwrap();

    let class_file = ClassFile {
        minor_version: 0,
        major_version: 52,
        constant_pool: vec![
            Box::new(ConstantPoolEntry::ConstantMethodref {
                class_index: 3,
                name_and_type_index: 6,
            }),
            Box::new(ConstantPoolEntry::ConstantClass {
                name_index: 7,
            }),
            Box::new(ConstantPoolEntry::ConstantClass {
                name_index: 8,
            }),
            Box::new(ConstantPoolEntry::ConstantUtf8 {
                string: "<init>".to_string(),
            }),
            Box::new(ConstantPoolEntry::ConstantUtf8 {
                string: "()V".to_string(),
            }),
            Box::new(ConstantPoolEntry::ConstantNameAndType {
                name_index: 4,
                descriptor_index: 5,
            }),
            Box::new(ConstantPoolEntry::ConstantUtf8 {
                string: "BadClazz".to_string(),
            }),
            Box::new(ConstantPoolEntry::ConstantUtf8 {
                string: "java/lang/Object".to_string(),
            }),
        ],
        access_flags: vec![
            ClassAccess::Public,
            ClassAccess::Super,
            ClassAccess::Enum,
            ClassAccess::Interface,
            ClassAccess::Module,
            ClassAccess::Final,
        ].into_iter().collect(),
        this_class: 2,
        super_class: 3,
        interfaces: vec![],
        fields: vec![],
        methods: vec![],
        attributes: vec![],
    };

    class_file.to_file(&mut file).unwrap();
}

