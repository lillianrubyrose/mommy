Ok(
    IRClassFile {
        magic: 3405691582,
        version: ClassFileVersion {
            major: 66,
            minor: 0,
        },
        cp: [
            MethodRef {
                class_index: 2,
                name_and_ty: CPNameAndTypeRef {
                    index: 3,
                    name: CPUtf8Ref {
                        data: "<init>",
                        index: 5,
                    },
                    ty: CPUtf8Ref {
                        data: "()V",
                        index: 6,
                    },
                },
            },
            Class(
                CPUtf8Ref {
                    data: "java/lang/Object",
                    index: 4,
                },
            ),
            NameAndType {
                name: CPUtf8Ref {
                    data: "<init>",
                    index: 5,
                },
                descriptor: CPUtf8Ref {
                    data: "()V",
                    index: 6,
                },
            },
            Utf8(
                "java/lang/Object",
            ),
            Utf8(
                "<init>",
            ),
            Utf8(
                "()V",
            ),
            FieldRef {
                class_index: 8,
                name_and_ty: CPNameAndTypeRef {
                    index: 9,
                    name: CPUtf8Ref {
                        data: "out",
                        index: 11,
                    },
                    ty: CPUtf8Ref {
                        data: "Ljava/io/PrintStream;",
                        index: 12,
                    },
                },
            },
            Class(
                CPUtf8Ref {
                    data: "java/lang/System",
                    index: 10,
                },
            ),
            NameAndType {
                name: CPUtf8Ref {
                    data: "out",
                    index: 11,
                },
                descriptor: CPUtf8Ref {
                    data: "Ljava/io/PrintStream;",
                    index: 12,
                },
            },
            Utf8(
                "java/lang/System",
            ),
            Utf8(
                "out",
            ),
            Utf8(
                "Ljava/io/PrintStream;",
            ),
            Class(
                CPUtf8Ref {
                    data: "Hello",
                    index: 14,
                },
            ),
            Utf8(
                "Hello",
            ),
            String(
                CPUtf8Ref {
                    data: "Hello World!",
                    index: 16,
                },
            ),
            Utf8(
                "Hello World!",
            ),
            MethodRef {
                class_index: 18,
                name_and_ty: CPNameAndTypeRef {
                    index: 19,
                    name: CPUtf8Ref {
                        data: "println",
                        index: 21,
                    },
                    ty: CPUtf8Ref {
                        data: "(Ljava/lang/String;)V",
                        index: 22,
                    },
                },
            },
            Class(
                CPUtf8Ref {
                    data: "java/io/PrintStream",
                    index: 20,
                },
            ),
            NameAndType {
                name: CPUtf8Ref {
                    data: "println",
                    index: 21,
                },
                descriptor: CPUtf8Ref {
                    data: "(Ljava/lang/String;)V",
                    index: 22,
                },
            },
            Utf8(
                "java/io/PrintStream",
            ),
            Utf8(
                "println",
            ),
            Utf8(
                "(Ljava/lang/String;)V",
            ),
            Utf8(
                "MESSAGE",
            ),
            Utf8(
                "Ljava/lang/String;",
            ),
            Utf8(
                "ConstantValue",
            ),
            Utf8(
                "Code",
            ),
            Utf8(
                "LineNumberTable",
            ),
            Utf8(
                "main",
            ),
            Utf8(
                "([Ljava/lang/String;)V",
            ),
            Utf8(
                "stackmapper",
            ),
            Utf8(
                "(I)V",
            ),
            Utf8(
                "StackMapTable",
            ),
            Utf8(
                "thrower",
            ),
            Utf8(
                "Exceptions",
            ),
            Class(
                CPUtf8Ref {
                    data: "java/lang/RuntimeException",
                    index: 36,
                },
            ),
            Utf8(
                "java/lang/RuntimeException",
            ),
            Utf8(
                "SourceFile",
            ),
            Utf8(
                "Hello.java",
            ),
            Utf8(
                "NestMembers",
            ),
            Class(
                CPUtf8Ref {
                    data: "Hello$InnerHello",
                    index: 41,
                },
            ),
            Utf8(
                "Hello$InnerHello",
            ),
            Class(
                CPUtf8Ref {
                    data: "Hello$HelloInterface",
                    index: 43,
                },
            ),
            Utf8(
                "Hello$HelloInterface",
            ),
            Utf8(
                "InnerClasses",
            ),
            Utf8(
                "InnerHello",
            ),
            Utf8(
                "HelloInterface",
            ),
        ],
        access_flags: 49,
        this_class: CPUtf8Ref {
            data: "Hello",
            index: 13,
        },
        super_class: CPUtf8Ref {
            data: "java/lang/Object",
            index: 2,
        },
        interfaces: [],
        fields: [
            IRFieldInfo {
                access_flags: 25,
                name: CPUtf8Ref {
                    data: "MESSAGE",
                    index: 23,
                },
                descriptor: CPUtf8Ref {
                    data: "Ljava/lang/String;",
                    index: 24,
                },
                attributes: [
                    IRAttributeInfo {
                        name: CPUtf8Ref {
                            data: "ConstantValue",
                            index: 25,
                        },
                        length: 2,
                        attr: ConstantValue(
                            String(
                                CPUtf8Ref {
                                    data: "Hello World!",
                                    index: 16,
                                },
                            ),
                        ),
                    },
                ],
            },
        ],
        methods: [
            IRMethodInfo {
                access_flags: 1,
                name: CPUtf8Ref {
                    data: "<init>",
                    index: 5,
                },
                descriptor: CPUtf8Ref {
                    data: "()V",
                    index: 6,
                },
                attributes: [
                    IRAttributeInfo {
                        name: CPUtf8Ref {
                            data: "Code",
                            index: 26,
                        },
                        length: 29,
                        attr: Deprecated,
                    },
                ],
            },
            IRMethodInfo {
                access_flags: 9,
                name: CPUtf8Ref {
                    data: "main",
                    index: 28,
                },
                descriptor: CPUtf8Ref {
                    data: "([Ljava/lang/String;)V",
                    index: 29,
                },
                attributes: [
                    IRAttributeInfo {
                        name: CPUtf8Ref {
                            data: "Code",
                            index: 26,
                        },
                        length: 37,
                        attr: Deprecated,
                    },
                ],
            },
            IRMethodInfo {
                access_flags: 9,
                name: CPUtf8Ref {
                    data: "stackmapper",
                    index: 30,
                },
                descriptor: CPUtf8Ref {
                    data: "(I)V",
                    index: 31,
                },
                attributes: [
                    IRAttributeInfo {
                        name: CPUtf8Ref {
                            data: "Code",
                            index: 26,
                        },
                        length: 100,
                        attr: Deprecated,
                    },
                ],
            },
            IRMethodInfo {
                access_flags: 1,
                name: CPUtf8Ref {
                    data: "thrower",
                    index: 33,
                },
                descriptor: CPUtf8Ref {
                    data: "()V",
                    index: 6,
                },
                attributes: [
                    IRAttributeInfo {
                        name: CPUtf8Ref {
                            data: "Code",
                            index: 26,
                        },
                        length: 25,
                        attr: Deprecated,
                    },
                    IRAttributeInfo {
                        name: CPUtf8Ref {
                            data: "Exceptions",
                            index: 34,
                        },
                        length: 4,
                        attr: Exceptions {
                            exception_index_table: [
                                CPUtf8Ref {
                                    data: "java/lang/RuntimeException",
                                    index: 35,
                                },
                            ],
                        },
                    },
                ],
            },
        ],
        attributes: [
            IRAttributeInfo {
                name: CPUtf8Ref {
                    data: "SourceFile",
                    index: 37,
                },
                length: 2,
                attr: Deprecated,
            },
            IRAttributeInfo {
                name: CPUtf8Ref {
                    data: "NestMembers",
                    index: 39,
                },
                length: 6,
                attr: Deprecated,
            },
            IRAttributeInfo {
                name: CPUtf8Ref {
                    data: "InnerClasses",
                    index: 44,
                },
                length: 18,
                attr: Deprecated,
            },
        ],
    },
)
