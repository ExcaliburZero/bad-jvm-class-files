extern crate jvm_class_file_parser;

use std::fs::File;

use jvm_class_file_parser::{
    Attribute, Bytecode, ClassAccess, ClassFile, Code, ConstantPoolEntry,
    Method
};

fn main() {
    let bad_class_files = vec![
        ("HelloWorld.class", hello_world()),
        ("EnumInterface.class", enum_interface()),
    ];

    for (filepath, ref class_file) in bad_class_files {
        create_class_file(filepath, class_file);

        println!("Wrote {}", filepath);
    }
}

fn create_class_file(filepath: &str, class_file: &ClassFile) {
    let mut file = File::create(filepath).unwrap();

    class_file.to_file(&mut file).unwrap();
}

fn hello_world() -> ClassFile {
    use Bytecode::*;
    use ConstantPoolEntry::*;

    ClassFile {
        minor_version: 0,
        major_version: 55,
        constant_pool: vec![
            Box::new(ConstantMethodref {
                class_index: 6,
                name_and_type_index: 15,
            }),
            Box::new(ConstantFieldref {
                class_index: 16,
                name_and_type_index: 17,
            }),
            Box::new(ConstantString {
                string_index: 18,
            }),
            Box::new(ConstantMethodref {
                class_index: 19,
                name_and_type_index: 20,
            }),
            Box::new(ConstantClass {
                name_index: 21,
            }),
            Box::new(ConstantClass {
                name_index: 22,
            }),
            Box::new(ConstantUtf8 {
                string: "<init>".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "()V".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "Code".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "LineNumberTable".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "main".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "([Ljava/lang/String;)V".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "SourceFile".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "HelloWorld.java".to_string(),
            }),
            Box::new(ConstantNameAndType {
                name_index: 7,
                descriptor_index: 8,
            }),
            Box::new(ConstantClass {
                name_index: 23,
            }),
            Box::new(ConstantNameAndType {
                name_index: 24,
                descriptor_index: 25,
            }),
            Box::new(ConstantUtf8 {
                string: "Hello, World!".to_string(),
            }),
            Box::new(ConstantClass {
                name_index: 26,
            }),
            Box::new(ConstantNameAndType {
                name_index: 27,
                descriptor_index: 28,
            }),
            Box::new(ConstantUtf8 {
                string: "HelloWorld".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "java/lang/Object".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "java/lang/System".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "out".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "Ljava/io/PrintStream;".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "java/io/PrintStream".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "println".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "(Ljava/lang/String;)V".to_string(),
            }),
        ],
        access_flags: vec![
            ClassAccess::Public,
            ClassAccess::Super,
        ].into_iter().collect(),
        this_class: 5,
        super_class: 6,
        interfaces: vec![],
        fields: vec![],
        methods: vec![
            Method {
                access_flags: 0x0009,
                name_index: 11,
                descriptor_index: 12,
                attributes: vec![
                    Attribute {
                        attribute_name_index: 9,
                        info: Code {
                            max_stack: 2,
                            max_locals: 1,
                            code: vec![
                                (0, Getstatic(2)),
                                (3, Ldc(3)),
                                (5, Invokevirtual(4)),
                                (8, Return),
                            ],
                            exception_table: vec![],
                            attributes: vec![],
                        }.to_bytes(),
                    }
                ],
            }
        ],
        attributes: vec![],
    }
}

fn enum_interface() -> ClassFile {
    use Bytecode::*;
    use ConstantPoolEntry::*;

    ClassFile {
        minor_version: 0,
        major_version: 55,
        constant_pool: vec![
            Box::new(ConstantMethodref {
                class_index: 6,
                name_and_type_index: 15,
            }),
            Box::new(ConstantFieldref {
                class_index: 16,
                name_and_type_index: 17,
            }),
            Box::new(ConstantString {
                string_index: 18,
            }),
            Box::new(ConstantMethodref {
                class_index: 19,
                name_and_type_index: 20,
            }),
            Box::new(ConstantClass {
                name_index: 21,
            }),
            Box::new(ConstantClass {
                name_index: 22,
            }),
            Box::new(ConstantUtf8 {
                string: "<init>".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "()V".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "Code".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "LineNumberTable".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "main".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "([Ljava/lang/String;)V".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "SourceFile".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "EnumInterface.java".to_string(),
            }),
            Box::new(ConstantNameAndType {
                name_index: 7,
                descriptor_index: 8,
            }),
            Box::new(ConstantClass {
                name_index: 23,
            }),
            Box::new(ConstantNameAndType {
                name_index: 24,
                descriptor_index: 25,
            }),
            Box::new(ConstantUtf8 {
                string: "Hello, World!".to_string(),
            }),
            Box::new(ConstantClass {
                name_index: 26,
            }),
            Box::new(ConstantNameAndType {
                name_index: 27,
                descriptor_index: 28,
            }),
            Box::new(ConstantUtf8 {
                string: "EnumInterface".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "java/lang/Object".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "java/lang/System".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "out".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "Ljava/io/PrintStream;".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "java/io/PrintStream".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "println".to_string(),
            }),
            Box::new(ConstantUtf8 {
                string: "(Ljava/lang/String;)V".to_string(),
            }),
        ],
        access_flags: vec![
            ClassAccess::Public,
            ClassAccess::Super,
            ClassAccess::Interface,
            ClassAccess::Enum,
        ].into_iter().collect(),
        this_class: 5,
        super_class: 6,
        interfaces: vec![],
        fields: vec![],
        methods: vec![
            Method {
                access_flags: 0x0009,
                name_index: 11,
                descriptor_index: 12,
                attributes: vec![
                    Attribute {
                        attribute_name_index: 9,
                        info: Code {
                            max_stack: 2,
                            max_locals: 1,
                            code: vec![
                                (0, Getstatic(2)),
                                (3, Ldc(3)),
                                (5, Invokevirtual(4)),
                                (8, Return),
                            ],
                            exception_table: vec![],
                            attributes: vec![],
                        }.to_bytes(),
                    }
                ],
            }
        ],
        attributes: vec![],
    }
}
