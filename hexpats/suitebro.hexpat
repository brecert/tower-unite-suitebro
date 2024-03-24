import std.core;
import std.mem;
import type.magic;
import type.guid;

namespace util {
    bool DISPLAY_DEBUG_BYTES in;

    fn warn(str fmt, auto ... args) {
        str message = builtin::std::format(fmt, args);
        str error = std::format("$[{:#04x}] WARN: {}", $, message);
        return std::print(error);
    };

    fn error(str fmt, auto ... args) {
        str message = builtin::std::format(fmt, args);
        str error = std::format("$[{:#04x}] ERROR: {}", $, message);
        return std::error(error);
    };

    fn debug(str fmt, auto ... args) {
        str message = builtin::std::format(fmt, args);
        str debug = std::format("$[{:#04x}] DEBUG: {}", $, message);
        return std::print(debug);
    };
    
    fn assert(bool condition, str fmt, auto ... args) {
        if (!condition) {
            str message = builtin::std::format(fmt, args);
            str error = std::format("$[{:#04x}] ERROR(assert): {}", $, message);
            return std::error(error);
        }
    };

    struct Lit<Type, auto ExpectedValue> {
        Type value;
        util::assert(value == ExpectedValue, "Expected \"{}\", got \"{}\"", ExpectedValue, value);
    } [[sealed, format("util::impl::format_lit")]];

    struct DebugBytes<auto Size> {
        if (DISPLAY_DEBUG_BYTES) {
            u8 debug_bytes[Size] @ $ [[sealed, color("ffffff")]];
        }
    };

    namespace impl {
        fn format_lit(ref auto object) {
            return object.value;
        };
    }
}

namespace gvas {
    // Normal Definitions

    struct string {
        u32 size;
        char data[size];

        // make it easier to write pattern matches and comparisons
        str value = std::mem::read_string($-size, size-1);
    } [[sealed, format("std::string::impl::format_sized_string"), transform("std::string::impl::format_sized_string")]];

    fn read_string() {
        u32 size @ $;
        str value = std::mem::read_string($+4, size-1);
        return value;
    };

    using coord = float;
    using guid = type::GUID;

    namespace type {
        struct LinearColor {
            coord r, g, b, a;
        };

        struct Vector {
            coord x, y, z;
        };

        struct Rotator {
            coord pitch, yaw, roll;
        };

        struct Quat {
            coord x, y, z, w;
        };

        struct DateTime {
            u64 ticks;
        };

        struct IntPoint {
            s32 x, y;
        };
    }

    namespace property {
        using seperator = util::Lit<u8, 0>;
        using Property;
    
        struct PropertyMap {
            Property properties[while(gvas::read_string() != "None")];
            string; // "None"
        };

        struct StrProperty<auto WithHeader> {
            if (WithHeader) {
                u64 length;
                seperator;
                char value[length];
            } else {
                string value;
            }
        };

        // Numbers
        struct NumberProperty<auto Name, T, auto WithHeader> {
            if (WithHeader) {
                u64 length;
                std::assert(length == sizeof(T), std::format("{}Property must have a length of {}", Name, sizeof(T)));
                seperator;
            }
            T value;
        } [[sealed, format("gvas::property::impl::format_with_value")]];

        struct FloatProperty<auto WithHeader> {
            NumberProperty<"Float", float, WithHeader> value;
        } [[sealed, format("gvas::property::impl::format_with_value")]];

        struct IntProperty<auto WithHeader> {
            NumberProperty<"Int", s32, WithHeader> value;
        } [[sealed, format("gvas::property::impl::format_with_value")]];

        struct StructType<auto Type> {
            match (Type) {
                // unreal types
                ("Vector"): type::Vector;
                ("Rotator"): type::Rotator;
                ("Quat"): type::Quat;
                ("DateTime"): type::DateTime;
                ("IntPoint"): type::IntPoint;
                ("Guid"): guid;
                ("LinearColor"): type::LinearColor;
                // unknown where this fits
                ("WorkshopFile"): u64;
                // custom types, while we could assume all custom types are PropertyMaps
                // there are unreal types that are unaccounted for and we want better errors for those
                ( "SteamID" 
                | "PlayerTrustSaveData"
                | "ItemPhysics"
                | "Transform"
                | "ItemSpawnDefaults"
                | "Colorable"
                | "WeatherManifestEntry"
                | "ItemConnectionData"
                ): PropertyMap;
                // unknown
                (_): {
                    // Copy/Pasted from PropertyType, keep synced.
                    // templates can't handle mutual recursion so that's why it's like this lol
                    util::error("Unimplemented StructType: {}", Type);
                }
            }
        };

        struct StructProperty {
            u64 length;
            string type_name;
            guid guid;
            seperator;
            
            util::DebugBytes<length> [[name("StructSize")]];
            StructType<type_name.value>;
        };

        enum ComponentType : u32 {
            Empty,
            RichText,
            SimpleText
        };

        // ArrayProperty only?
        struct TextProperty {
            ComponentType type [[color("ff9933")]];
            seperator;

            match (type) {
                (ComponentType::Empty): {
                    util::error("Unhandled TextProperty type: {}", type);
                }
                (ComponentType::RichText): {
                    util::error("Unhandled TextProperty type: {}", type);
                }
                (ComponentType::SimpleText): {
                    u64 unk1;
                    string;
                }
            }
        };

        // // ArrayProperty is not supported in arrays
        struct ArrayProperty<auto WithHeader> {
            if (!WithHeader) {
                util::error("ArrayProperty cannot be used without a header");
            }

            u64 length;
            string type;
            seperator;
            util::DebugBytes<length> [[name("ArraySize")]];

            u32 property_count;

            match (type.value) {
                ("StructProperty"): {
                    string field_name;
                    util::Lit<string, type>;
                    u64 properties_size;
                    
                    string struct_name;
                    guid guid;
                    seperator;
                    
                    util::DebugBytes<properties_size> [[name("PropertiesSize")]];
                    StructType<struct_name.value> properties[property_count];
                }
                ("BoolProperty"): {
                    bool properties[property_count];
                }
                ("TextProperty"): {
                    TextProperty properties[property_count];
                }
                ("StrProperty"): {
                    string properties[property_count];
                }
                (_): {
                    util::error("Unhandled ArayProperty type: {}", type);
                }
            }
        };
        
        struct NameProperty<auto WithHeader> {
            if (WithHeader) {
                u64 length;
                seperator;
            }
            string value;
        };
        
        struct BoolProperty<auto WithHeader> {
            if (WithHeader) {
                u64 length;
            }
            bool value;
            if (WithHeader) {
                u8 indicator;
            }
        };
        
        struct EnumProperty {
            u64 length;
            string enum_type;
            seperator;
            string value;
        };
        
        struct ByteProperty<auto WithHeader> {
            if (!WithHeader) util::error("ByteProperty does not currently support parsing without a header");
            u64 size;
            string name;
            seperator;
            
            u8 value[size];
        };
        
        struct ObjectProperty<auto WithHeader> {
            if (WithHeader) {
                u64 size;
                seperator;
            }
            string value;
        };

        // Property

        struct PropertyType<auto Type, auto WithHeader> {
            match (Type) {
                ("FloatProperty"): FloatProperty<WithHeader>;
                ("ArrayProperty"): ArrayProperty<WithHeader>;
                ("StructProperty"): StructProperty;
                ("NameProperty"): NameProperty<WithHeader>;
                ("BoolProperty"): BoolProperty<WithHeader>;
                ("StrProperty"): StrProperty<WithHeader>;
                ("IntProperty"): IntProperty<WithHeader>;
                ("EnumProperty"): EnumProperty;
                ("ByteProperty"): ByteProperty<WithHeader>;
                ("ObjectProperty"): ObjectProperty<WithHeader>;
                (_): util::error("Unhandled PropertyType: {}", Type);
            }
        };
        

        struct Property {
            string name;

            match (name.value) {
                ("None"): return;
                (_): {
                    string type;
                    PropertyType<type.value, true> value;
                }
            }
        };

        namespace impl {
            fn format_with_value(ref auto object) {
                return std::format("{}", object.value);
            };
        }
    }
}

namespace suitebro {
    struct Header {
        type::Magic<"suitebro">;
        u32 format_version;
        u32 unreal_version;
    };

    struct PropertySection {
        gvas::string name;
        gvas::property::PropertyMap properties;
        u32;
    };
    
    struct Item {
        gvas::string name;
        gvas::guid guid;
        u32 unk_has_state;
        u64 steam_item_id;
        u32 tinyrick_size;
        u128 expected_end = $ + tinyrick_size; 
        
        type::Magic<"tinyrick">;
        u32 format_version;
        u32 unreal_version;
        
        gvas::property::PropertyMap properties;
        
        u32 unk1;
        u32 unk2;

        PropertySection p[unk2];
        
        if ($ != expected_end) {
            util::warn("tinyrick size was expected to end at {}, found end at {} instead", expected_end, $);
            $ = expected_end;
        }
        
        gvas::type::Quat rotation;
        gvas::type::Vector position;
        gvas::type::Vector scale;
    };

    struct PropertyList {
        gvas::string name;
        u32 size;
        util::DebugBytes<size>;
        u128 end = size+$;
        gvas::property::Property properties[while($+4 < end)];
        util::Lit<u32, 0>;
    };
    
    struct GroupInfo {
        u32 item_count;
        u32 group_id;
    };
    
    struct SuiteBro {
        Header header;
        
        u32 item_count;
        Item items[item_count];

        u32 property_count;
        PropertyList properties[property_count];

        u32;
        u32 group_count;
        GroupInfo groups[group_count];
    };
}

suitebro::SuiteBro file @ 0x00 [[inline]];