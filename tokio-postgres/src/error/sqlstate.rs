// Autogenerated file - DO NOT EDIT
use std::borrow::Cow;

/// A SQLSTATE error code
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SqlState(Cow<'static, str>);

impl SqlState {
    /// Creates a `SqlState` from its error code.
    pub fn from_code(s: &str) -> SqlState {
        match SQLSTATE_MAP.get(s) {
            Some(state) => state.clone(),
            None => SqlState(Cow::Owned(s.to_string())),
        }
    }

    /// Returns the error code corresponding to the `SqlState`.
    pub fn code(&self) -> &str {
        &self.0
    }

    /// 00000
    pub const SUCCESSFUL_COMPLETION: SqlState = SqlState(Cow::Borrowed("00000"));

    /// 01000
    pub const WARNING: SqlState = SqlState(Cow::Borrowed("01000"));

    /// 0100C
    pub const WARNING_DYNAMIC_RESULT_SETS_RETURNED: SqlState = SqlState(Cow::Borrowed("0100C"));

    /// 01008
    pub const WARNING_IMPLICIT_ZERO_BIT_PADDING: SqlState = SqlState(Cow::Borrowed("01008"));

    /// 01003
    pub const WARNING_NULL_VALUE_ELIMINATED_IN_SET_FUNCTION: SqlState =
        SqlState(Cow::Borrowed("01003"));

    /// 01007
    pub const WARNING_PRIVILEGE_NOT_GRANTED: SqlState = SqlState(Cow::Borrowed("01007"));

    /// 01006
    pub const WARNING_PRIVILEGE_NOT_REVOKED: SqlState = SqlState(Cow::Borrowed("01006"));

    /// 01004
    pub const WARNING_STRING_DATA_RIGHT_TRUNCATION: SqlState = SqlState(Cow::Borrowed("01004"));

    /// 01P01
    pub const WARNING_DEPRECATED_FEATURE: SqlState = SqlState(Cow::Borrowed("01P01"));

    /// 02000
    pub const NO_DATA: SqlState = SqlState(Cow::Borrowed("02000"));

    /// 02001
    pub const NO_ADDITIONAL_DYNAMIC_RESULT_SETS_RETURNED: SqlState =
        SqlState(Cow::Borrowed("02001"));

    /// 03000
    pub const SQL_STATEMENT_NOT_YET_COMPLETE: SqlState = SqlState(Cow::Borrowed("03000"));

    /// 08000
    pub const CONNECTION_EXCEPTION: SqlState = SqlState(Cow::Borrowed("08000"));

    /// 08003
    pub const CONNECTION_DOES_NOT_EXIST: SqlState = SqlState(Cow::Borrowed("08003"));

    /// 08006
    pub const CONNECTION_FAILURE: SqlState = SqlState(Cow::Borrowed("08006"));

    /// 08001
    pub const SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION: SqlState =
        SqlState(Cow::Borrowed("08001"));

    /// 08004
    pub const SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION: SqlState =
        SqlState(Cow::Borrowed("08004"));

    /// 08007
    pub const TRANSACTION_RESOLUTION_UNKNOWN: SqlState = SqlState(Cow::Borrowed("08007"));

    /// 08P01
    pub const PROTOCOL_VIOLATION: SqlState = SqlState(Cow::Borrowed("08P01"));

    /// 09000
    pub const TRIGGERED_ACTION_EXCEPTION: SqlState = SqlState(Cow::Borrowed("09000"));

    /// 0A000
    pub const FEATURE_NOT_SUPPORTED: SqlState = SqlState(Cow::Borrowed("0A000"));

    /// 0B000
    pub const INVALID_TRANSACTION_INITIATION: SqlState = SqlState(Cow::Borrowed("0B000"));

    /// 0F000
    pub const LOCATOR_EXCEPTION: SqlState = SqlState(Cow::Borrowed("0F000"));

    /// 0F001
    pub const L_E_INVALID_SPECIFICATION: SqlState = SqlState(Cow::Borrowed("0F001"));

    /// 0L000
    pub const INVALID_GRANTOR: SqlState = SqlState(Cow::Borrowed("0L000"));

    /// 0LP01
    pub const INVALID_GRANT_OPERATION: SqlState = SqlState(Cow::Borrowed("0LP01"));

    /// 0P000
    pub const INVALID_ROLE_SPECIFICATION: SqlState = SqlState(Cow::Borrowed("0P000"));

    /// 0Z000
    pub const DIAGNOSTICS_EXCEPTION: SqlState = SqlState(Cow::Borrowed("0Z000"));

    /// 0Z002
    pub const STACKED_DIAGNOSTICS_ACCESSED_WITHOUT_ACTIVE_HANDLER: SqlState =
        SqlState(Cow::Borrowed("0Z002"));

    /// 20000
    pub const CASE_NOT_FOUND: SqlState = SqlState(Cow::Borrowed("20000"));

    /// 21000
    pub const CARDINALITY_VIOLATION: SqlState = SqlState(Cow::Borrowed("21000"));

    /// 22000
    pub const DATA_EXCEPTION: SqlState = SqlState(Cow::Borrowed("22000"));

    /// 2202E
    pub const ARRAY_ELEMENT_ERROR: SqlState = SqlState(Cow::Borrowed("2202E"));

    /// 2202E
    pub const ARRAY_SUBSCRIPT_ERROR: SqlState = SqlState(Cow::Borrowed("2202E"));

    /// 22021
    pub const CHARACTER_NOT_IN_REPERTOIRE: SqlState = SqlState(Cow::Borrowed("22021"));

    /// 22008
    pub const DATETIME_FIELD_OVERFLOW: SqlState = SqlState(Cow::Borrowed("22008"));

    /// 22008
    pub const DATETIME_VALUE_OUT_OF_RANGE: SqlState = SqlState(Cow::Borrowed("22008"));

    /// 22012
    pub const DIVISION_BY_ZERO: SqlState = SqlState(Cow::Borrowed("22012"));

    /// 22005
    pub const ERROR_IN_ASSIGNMENT: SqlState = SqlState(Cow::Borrowed("22005"));

    /// 2200B
    pub const ESCAPE_CHARACTER_CONFLICT: SqlState = SqlState(Cow::Borrowed("2200B"));

    /// 22022
    pub const INDICATOR_OVERFLOW: SqlState = SqlState(Cow::Borrowed("22022"));

    /// 22015
    pub const INTERVAL_FIELD_OVERFLOW: SqlState = SqlState(Cow::Borrowed("22015"));

    /// 2201E
    pub const INVALID_ARGUMENT_FOR_LOG: SqlState = SqlState(Cow::Borrowed("2201E"));

    /// 22014
    pub const INVALID_ARGUMENT_FOR_NTILE: SqlState = SqlState(Cow::Borrowed("22014"));

    /// 22016
    pub const INVALID_ARGUMENT_FOR_NTH_VALUE: SqlState = SqlState(Cow::Borrowed("22016"));

    /// 2201F
    pub const INVALID_ARGUMENT_FOR_POWER_FUNCTION: SqlState = SqlState(Cow::Borrowed("2201F"));

    /// 2201G
    pub const INVALID_ARGUMENT_FOR_WIDTH_BUCKET_FUNCTION: SqlState =
        SqlState(Cow::Borrowed("2201G"));

    /// 22018
    pub const INVALID_CHARACTER_VALUE_FOR_CAST: SqlState = SqlState(Cow::Borrowed("22018"));

    /// 22007
    pub const INVALID_DATETIME_FORMAT: SqlState = SqlState(Cow::Borrowed("22007"));

    /// 22019
    pub const INVALID_ESCAPE_CHARACTER: SqlState = SqlState(Cow::Borrowed("22019"));

    /// 2200D
    pub const INVALID_ESCAPE_OCTET: SqlState = SqlState(Cow::Borrowed("2200D"));

    /// 22025
    pub const INVALID_ESCAPE_SEQUENCE: SqlState = SqlState(Cow::Borrowed("22025"));

    /// 22P06
    pub const NONSTANDARD_USE_OF_ESCAPE_CHARACTER: SqlState = SqlState(Cow::Borrowed("22P06"));

    /// 22010
    pub const INVALID_INDICATOR_PARAMETER_VALUE: SqlState = SqlState(Cow::Borrowed("22010"));

    /// 22023
    pub const INVALID_PARAMETER_VALUE: SqlState = SqlState(Cow::Borrowed("22023"));

    /// 2201B
    pub const INVALID_REGULAR_EXPRESSION: SqlState = SqlState(Cow::Borrowed("2201B"));

    /// 2201W
    pub const INVALID_ROW_COUNT_IN_LIMIT_CLAUSE: SqlState = SqlState(Cow::Borrowed("2201W"));

    /// 2201X
    pub const INVALID_ROW_COUNT_IN_RESULT_OFFSET_CLAUSE: SqlState =
        SqlState(Cow::Borrowed("2201X"));

    /// 2202H
    pub const INVALID_TABLESAMPLE_ARGUMENT: SqlState = SqlState(Cow::Borrowed("2202H"));

    /// 2202G
    pub const INVALID_TABLESAMPLE_REPEAT: SqlState = SqlState(Cow::Borrowed("2202G"));

    /// 22009
    pub const INVALID_TIME_ZONE_DISPLACEMENT_VALUE: SqlState = SqlState(Cow::Borrowed("22009"));

    /// 2200C
    pub const INVALID_USE_OF_ESCAPE_CHARACTER: SqlState = SqlState(Cow::Borrowed("2200C"));

    /// 2200G
    pub const MOST_SPECIFIC_TYPE_MISMATCH: SqlState = SqlState(Cow::Borrowed("2200G"));

    /// 22004
    pub const NULL_VALUE_NOT_ALLOWED: SqlState = SqlState(Cow::Borrowed("22004"));

    /// 22002
    pub const NULL_VALUE_NO_INDICATOR_PARAMETER: SqlState = SqlState(Cow::Borrowed("22002"));

    /// 22003
    pub const NUMERIC_VALUE_OUT_OF_RANGE: SqlState = SqlState(Cow::Borrowed("22003"));

    /// 2200H
    pub const SEQUENCE_GENERATOR_LIMIT_EXCEEDED: SqlState = SqlState(Cow::Borrowed("2200H"));

    /// 22026
    pub const STRING_DATA_LENGTH_MISMATCH: SqlState = SqlState(Cow::Borrowed("22026"));

    /// 22001
    pub const STRING_DATA_RIGHT_TRUNCATION: SqlState = SqlState(Cow::Borrowed("22001"));

    /// 22011
    pub const SUBSTRING_ERROR: SqlState = SqlState(Cow::Borrowed("22011"));

    /// 22027
    pub const TRIM_ERROR: SqlState = SqlState(Cow::Borrowed("22027"));

    /// 22024
    pub const UNTERMINATED_C_STRING: SqlState = SqlState(Cow::Borrowed("22024"));

    /// 2200F
    pub const ZERO_LENGTH_CHARACTER_STRING: SqlState = SqlState(Cow::Borrowed("2200F"));

    /// 22P01
    pub const FLOATING_POINT_EXCEPTION: SqlState = SqlState(Cow::Borrowed("22P01"));

    /// 22P02
    pub const INVALID_TEXT_REPRESENTATION: SqlState = SqlState(Cow::Borrowed("22P02"));

    /// 22P03
    pub const INVALID_BINARY_REPRESENTATION: SqlState = SqlState(Cow::Borrowed("22P03"));

    /// 22P04
    pub const BAD_COPY_FILE_FORMAT: SqlState = SqlState(Cow::Borrowed("22P04"));

    /// 22P05
    pub const UNTRANSLATABLE_CHARACTER: SqlState = SqlState(Cow::Borrowed("22P05"));

    /// 2200L
    pub const NOT_AN_XML_DOCUMENT: SqlState = SqlState(Cow::Borrowed("2200L"));

    /// 2200M
    pub const INVALID_XML_DOCUMENT: SqlState = SqlState(Cow::Borrowed("2200M"));

    /// 2200N
    pub const INVALID_XML_CONTENT: SqlState = SqlState(Cow::Borrowed("2200N"));

    /// 2200S
    pub const INVALID_XML_COMMENT: SqlState = SqlState(Cow::Borrowed("2200S"));

    /// 2200T
    pub const INVALID_XML_PROCESSING_INSTRUCTION: SqlState = SqlState(Cow::Borrowed("2200T"));

    /// 23000
    pub const INTEGRITY_CONSTRAINT_VIOLATION: SqlState = SqlState(Cow::Borrowed("23000"));

    /// 23001
    pub const RESTRICT_VIOLATION: SqlState = SqlState(Cow::Borrowed("23001"));

    /// 23502
    pub const NOT_NULL_VIOLATION: SqlState = SqlState(Cow::Borrowed("23502"));

    /// 23503
    pub const FOREIGN_KEY_VIOLATION: SqlState = SqlState(Cow::Borrowed("23503"));

    /// 23505
    pub const UNIQUE_VIOLATION: SqlState = SqlState(Cow::Borrowed("23505"));

    /// 23514
    pub const CHECK_VIOLATION: SqlState = SqlState(Cow::Borrowed("23514"));

    /// 23P01
    pub const EXCLUSION_VIOLATION: SqlState = SqlState(Cow::Borrowed("23P01"));

    /// 24000
    pub const INVALID_CURSOR_STATE: SqlState = SqlState(Cow::Borrowed("24000"));

    /// 25000
    pub const INVALID_TRANSACTION_STATE: SqlState = SqlState(Cow::Borrowed("25000"));

    /// 25001
    pub const ACTIVE_SQL_TRANSACTION: SqlState = SqlState(Cow::Borrowed("25001"));

    /// 25002
    pub const BRANCH_TRANSACTION_ALREADY_ACTIVE: SqlState = SqlState(Cow::Borrowed("25002"));

    /// 25008
    pub const HELD_CURSOR_REQUIRES_SAME_ISOLATION_LEVEL: SqlState =
        SqlState(Cow::Borrowed("25008"));

    /// 25003
    pub const INAPPROPRIATE_ACCESS_MODE_FOR_BRANCH_TRANSACTION: SqlState =
        SqlState(Cow::Borrowed("25003"));

    /// 25004
    pub const INAPPROPRIATE_ISOLATION_LEVEL_FOR_BRANCH_TRANSACTION: SqlState =
        SqlState(Cow::Borrowed("25004"));

    /// 25005
    pub const NO_ACTIVE_SQL_TRANSACTION_FOR_BRANCH_TRANSACTION: SqlState =
        SqlState(Cow::Borrowed("25005"));

    /// 25006
    pub const READ_ONLY_SQL_TRANSACTION: SqlState = SqlState(Cow::Borrowed("25006"));

    /// 25007
    pub const SCHEMA_AND_DATA_STATEMENT_MIXING_NOT_SUPPORTED: SqlState =
        SqlState(Cow::Borrowed("25007"));

    /// 25P01
    pub const NO_ACTIVE_SQL_TRANSACTION: SqlState = SqlState(Cow::Borrowed("25P01"));

    /// 25P02
    pub const IN_FAILED_SQL_TRANSACTION: SqlState = SqlState(Cow::Borrowed("25P02"));

    /// 25P03
    pub const IDLE_IN_TRANSACTION_SESSION_TIMEOUT: SqlState = SqlState(Cow::Borrowed("25P03"));

    /// 26000
    pub const INVALID_SQL_STATEMENT_NAME: SqlState = SqlState(Cow::Borrowed("26000"));

    /// 26000
    pub const UNDEFINED_PSTATEMENT: SqlState = SqlState(Cow::Borrowed("26000"));

    /// 27000
    pub const TRIGGERED_DATA_CHANGE_VIOLATION: SqlState = SqlState(Cow::Borrowed("27000"));

    /// 28000
    pub const INVALID_AUTHORIZATION_SPECIFICATION: SqlState = SqlState(Cow::Borrowed("28000"));

    /// 28P01
    pub const INVALID_PASSWORD: SqlState = SqlState(Cow::Borrowed("28P01"));

    /// 2B000
    pub const DEPENDENT_PRIVILEGE_DESCRIPTORS_STILL_EXIST: SqlState =
        SqlState(Cow::Borrowed("2B000"));

    /// 2BP01
    pub const DEPENDENT_OBJECTS_STILL_EXIST: SqlState = SqlState(Cow::Borrowed("2BP01"));

    /// 2D000
    pub const INVALID_TRANSACTION_TERMINATION: SqlState = SqlState(Cow::Borrowed("2D000"));

    /// 2F000
    pub const SQL_ROUTINE_EXCEPTION: SqlState = SqlState(Cow::Borrowed("2F000"));

    /// 2F005
    pub const S_R_E_FUNCTION_EXECUTED_NO_RETURN_STATEMENT: SqlState =
        SqlState(Cow::Borrowed("2F005"));

    /// 2F002
    pub const S_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED: SqlState = SqlState(Cow::Borrowed("2F002"));

    /// 2F003
    pub const S_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED: SqlState = SqlState(Cow::Borrowed("2F003"));

    /// 2F004
    pub const S_R_E_READING_SQL_DATA_NOT_PERMITTED: SqlState = SqlState(Cow::Borrowed("2F004"));

    /// 34000
    pub const INVALID_CURSOR_NAME: SqlState = SqlState(Cow::Borrowed("34000"));

    /// 34000
    pub const UNDEFINED_CURSOR: SqlState = SqlState(Cow::Borrowed("34000"));

    /// 38000
    pub const EXTERNAL_ROUTINE_EXCEPTION: SqlState = SqlState(Cow::Borrowed("38000"));

    /// 38001
    pub const E_R_E_CONTAINING_SQL_NOT_PERMITTED: SqlState = SqlState(Cow::Borrowed("38001"));

    /// 38002
    pub const E_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED: SqlState = SqlState(Cow::Borrowed("38002"));

    /// 38003
    pub const E_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED: SqlState = SqlState(Cow::Borrowed("38003"));

    /// 38004
    pub const E_R_E_READING_SQL_DATA_NOT_PERMITTED: SqlState = SqlState(Cow::Borrowed("38004"));

    /// 39000
    pub const EXTERNAL_ROUTINE_INVOCATION_EXCEPTION: SqlState = SqlState(Cow::Borrowed("39000"));

    /// 39001
    pub const E_R_I_E_INVALID_SQLSTATE_RETURNED: SqlState = SqlState(Cow::Borrowed("39001"));

    /// 39004
    pub const E_R_I_E_NULL_VALUE_NOT_ALLOWED: SqlState = SqlState(Cow::Borrowed("39004"));

    /// 39P01
    pub const E_R_I_E_TRIGGER_PROTOCOL_VIOLATED: SqlState = SqlState(Cow::Borrowed("39P01"));

    /// 39P02
    pub const E_R_I_E_SRF_PROTOCOL_VIOLATED: SqlState = SqlState(Cow::Borrowed("39P02"));

    /// 39P03
    pub const E_R_I_E_EVENT_TRIGGER_PROTOCOL_VIOLATED: SqlState = SqlState(Cow::Borrowed("39P03"));

    /// 3B000
    pub const SAVEPOINT_EXCEPTION: SqlState = SqlState(Cow::Borrowed("3B000"));

    /// 3B001
    pub const S_E_INVALID_SPECIFICATION: SqlState = SqlState(Cow::Borrowed("3B001"));

    /// 3D000
    pub const INVALID_CATALOG_NAME: SqlState = SqlState(Cow::Borrowed("3D000"));

    /// 3D000
    pub const UNDEFINED_DATABASE: SqlState = SqlState(Cow::Borrowed("3D000"));

    /// 3F000
    pub const INVALID_SCHEMA_NAME: SqlState = SqlState(Cow::Borrowed("3F000"));

    /// 3F000
    pub const UNDEFINED_SCHEMA: SqlState = SqlState(Cow::Borrowed("3F000"));

    /// 40000
    pub const TRANSACTION_ROLLBACK: SqlState = SqlState(Cow::Borrowed("40000"));

    /// 40002
    pub const T_R_INTEGRITY_CONSTRAINT_VIOLATION: SqlState = SqlState(Cow::Borrowed("40002"));

    /// 40001
    pub const T_R_SERIALIZATION_FAILURE: SqlState = SqlState(Cow::Borrowed("40001"));

    /// 40003
    pub const T_R_STATEMENT_COMPLETION_UNKNOWN: SqlState = SqlState(Cow::Borrowed("40003"));

    /// 40P01
    pub const T_R_DEADLOCK_DETECTED: SqlState = SqlState(Cow::Borrowed("40P01"));

    /// 42000
    pub const SYNTAX_ERROR_OR_ACCESS_RULE_VIOLATION: SqlState = SqlState(Cow::Borrowed("42000"));

    /// 42601
    pub const SYNTAX_ERROR: SqlState = SqlState(Cow::Borrowed("42601"));

    /// 42501
    pub const INSUFFICIENT_PRIVILEGE: SqlState = SqlState(Cow::Borrowed("42501"));

    /// 42846
    pub const CANNOT_COERCE: SqlState = SqlState(Cow::Borrowed("42846"));

    /// 42803
    pub const GROUPING_ERROR: SqlState = SqlState(Cow::Borrowed("42803"));

    /// 42P20
    pub const WINDOWING_ERROR: SqlState = SqlState(Cow::Borrowed("42P20"));

    /// 42P19
    pub const INVALID_RECURSION: SqlState = SqlState(Cow::Borrowed("42P19"));

    /// 42830
    pub const INVALID_FOREIGN_KEY: SqlState = SqlState(Cow::Borrowed("42830"));

    /// 42602
    pub const INVALID_NAME: SqlState = SqlState(Cow::Borrowed("42602"));

    /// 42622
    pub const NAME_TOO_LONG: SqlState = SqlState(Cow::Borrowed("42622"));

    /// 42939
    pub const RESERVED_NAME: SqlState = SqlState(Cow::Borrowed("42939"));

    /// 42804
    pub const DATATYPE_MISMATCH: SqlState = SqlState(Cow::Borrowed("42804"));

    /// 42P18
    pub const INDETERMINATE_DATATYPE: SqlState = SqlState(Cow::Borrowed("42P18"));

    /// 42P21
    pub const COLLATION_MISMATCH: SqlState = SqlState(Cow::Borrowed("42P21"));

    /// 42P22
    pub const INDETERMINATE_COLLATION: SqlState = SqlState(Cow::Borrowed("42P22"));

    /// 42809
    pub const WRONG_OBJECT_TYPE: SqlState = SqlState(Cow::Borrowed("42809"));

    /// 428C9
    pub const GENERATED_ALWAYS: SqlState = SqlState(Cow::Borrowed("428C9"));

    /// 42703
    pub const UNDEFINED_COLUMN: SqlState = SqlState(Cow::Borrowed("42703"));

    /// 42883
    pub const UNDEFINED_FUNCTION: SqlState = SqlState(Cow::Borrowed("42883"));

    /// 42P01
    pub const UNDEFINED_TABLE: SqlState = SqlState(Cow::Borrowed("42P01"));

    /// 42P02
    pub const UNDEFINED_PARAMETER: SqlState = SqlState(Cow::Borrowed("42P02"));

    /// 42704
    pub const UNDEFINED_OBJECT: SqlState = SqlState(Cow::Borrowed("42704"));

    /// 42701
    pub const DUPLICATE_COLUMN: SqlState = SqlState(Cow::Borrowed("42701"));

    /// 42P03
    pub const DUPLICATE_CURSOR: SqlState = SqlState(Cow::Borrowed("42P03"));

    /// 42P04
    pub const DUPLICATE_DATABASE: SqlState = SqlState(Cow::Borrowed("42P04"));

    /// 42723
    pub const DUPLICATE_FUNCTION: SqlState = SqlState(Cow::Borrowed("42723"));

    /// 42P05
    pub const DUPLICATE_PSTATEMENT: SqlState = SqlState(Cow::Borrowed("42P05"));

    /// 42P06
    pub const DUPLICATE_SCHEMA: SqlState = SqlState(Cow::Borrowed("42P06"));

    /// 42P07
    pub const DUPLICATE_TABLE: SqlState = SqlState(Cow::Borrowed("42P07"));

    /// 42712
    pub const DUPLICATE_ALIAS: SqlState = SqlState(Cow::Borrowed("42712"));

    /// 42710
    pub const DUPLICATE_OBJECT: SqlState = SqlState(Cow::Borrowed("42710"));

    /// 42702
    pub const AMBIGUOUS_COLUMN: SqlState = SqlState(Cow::Borrowed("42702"));

    /// 42725
    pub const AMBIGUOUS_FUNCTION: SqlState = SqlState(Cow::Borrowed("42725"));

    /// 42P08
    pub const AMBIGUOUS_PARAMETER: SqlState = SqlState(Cow::Borrowed("42P08"));

    /// 42P09
    pub const AMBIGUOUS_ALIAS: SqlState = SqlState(Cow::Borrowed("42P09"));

    /// 42P10
    pub const INVALID_COLUMN_REFERENCE: SqlState = SqlState(Cow::Borrowed("42P10"));

    /// 42611
    pub const INVALID_COLUMN_DEFINITION: SqlState = SqlState(Cow::Borrowed("42611"));

    /// 42P11
    pub const INVALID_CURSOR_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P11"));

    /// 42P12
    pub const INVALID_DATABASE_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P12"));

    /// 42P13
    pub const INVALID_FUNCTION_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P13"));

    /// 42P14
    pub const INVALID_PSTATEMENT_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P14"));

    /// 42P15
    pub const INVALID_SCHEMA_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P15"));

    /// 42P16
    pub const INVALID_TABLE_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P16"));

    /// 42P17
    pub const INVALID_OBJECT_DEFINITION: SqlState = SqlState(Cow::Borrowed("42P17"));

    /// 44000
    pub const WITH_CHECK_OPTION_VIOLATION: SqlState = SqlState(Cow::Borrowed("44000"));

    /// 53000
    pub const INSUFFICIENT_RESOURCES: SqlState = SqlState(Cow::Borrowed("53000"));

    /// 53100
    pub const DISK_FULL: SqlState = SqlState(Cow::Borrowed("53100"));

    /// 53200
    pub const OUT_OF_MEMORY: SqlState = SqlState(Cow::Borrowed("53200"));

    /// 53300
    pub const TOO_MANY_CONNECTIONS: SqlState = SqlState(Cow::Borrowed("53300"));

    /// 53400
    pub const CONFIGURATION_LIMIT_EXCEEDED: SqlState = SqlState(Cow::Borrowed("53400"));

    /// 54000
    pub const PROGRAM_LIMIT_EXCEEDED: SqlState = SqlState(Cow::Borrowed("54000"));

    /// 54001
    pub const STATEMENT_TOO_COMPLEX: SqlState = SqlState(Cow::Borrowed("54001"));

    /// 54011
    pub const TOO_MANY_COLUMNS: SqlState = SqlState(Cow::Borrowed("54011"));

    /// 54023
    pub const TOO_MANY_ARGUMENTS: SqlState = SqlState(Cow::Borrowed("54023"));

    /// 55000
    pub const OBJECT_NOT_IN_PREREQUISITE_STATE: SqlState = SqlState(Cow::Borrowed("55000"));

    /// 55006
    pub const OBJECT_IN_USE: SqlState = SqlState(Cow::Borrowed("55006"));

    /// 55P02
    pub const CANT_CHANGE_RUNTIME_PARAM: SqlState = SqlState(Cow::Borrowed("55P02"));

    /// 55P03
    pub const LOCK_NOT_AVAILABLE: SqlState = SqlState(Cow::Borrowed("55P03"));

    /// 55P04
    pub const UNSAFE_NEW_ENUM_VALUE_USAGE: SqlState = SqlState(Cow::Borrowed("55P04"));

    /// 57000
    pub const OPERATOR_INTERVENTION: SqlState = SqlState(Cow::Borrowed("57000"));

    /// 57014
    pub const QUERY_CANCELED: SqlState = SqlState(Cow::Borrowed("57014"));

    /// 57P01
    pub const ADMIN_SHUTDOWN: SqlState = SqlState(Cow::Borrowed("57P01"));

    /// 57P02
    pub const CRASH_SHUTDOWN: SqlState = SqlState(Cow::Borrowed("57P02"));

    /// 57P03
    pub const CANNOT_CONNECT_NOW: SqlState = SqlState(Cow::Borrowed("57P03"));

    /// 57P04
    pub const DATABASE_DROPPED: SqlState = SqlState(Cow::Borrowed("57P04"));

    /// 58000
    pub const SYSTEM_ERROR: SqlState = SqlState(Cow::Borrowed("58000"));

    /// 58030
    pub const IO_ERROR: SqlState = SqlState(Cow::Borrowed("58030"));

    /// 58P01
    pub const UNDEFINED_FILE: SqlState = SqlState(Cow::Borrowed("58P01"));

    /// 58P02
    pub const DUPLICATE_FILE: SqlState = SqlState(Cow::Borrowed("58P02"));

    /// 72000
    pub const SNAPSHOT_TOO_OLD: SqlState = SqlState(Cow::Borrowed("72000"));

    /// F0000
    pub const CONFIG_FILE_ERROR: SqlState = SqlState(Cow::Borrowed("F0000"));

    /// F0001
    pub const LOCK_FILE_EXISTS: SqlState = SqlState(Cow::Borrowed("F0001"));

    /// HV000
    pub const FDW_ERROR: SqlState = SqlState(Cow::Borrowed("HV000"));

    /// HV005
    pub const FDW_COLUMN_NAME_NOT_FOUND: SqlState = SqlState(Cow::Borrowed("HV005"));

    /// HV002
    pub const FDW_DYNAMIC_PARAMETER_VALUE_NEEDED: SqlState = SqlState(Cow::Borrowed("HV002"));

    /// HV010
    pub const FDW_FUNCTION_SEQUENCE_ERROR: SqlState = SqlState(Cow::Borrowed("HV010"));

    /// HV021
    pub const FDW_INCONSISTENT_DESCRIPTOR_INFORMATION: SqlState = SqlState(Cow::Borrowed("HV021"));

    /// HV024
    pub const FDW_INVALID_ATTRIBUTE_VALUE: SqlState = SqlState(Cow::Borrowed("HV024"));

    /// HV007
    pub const FDW_INVALID_COLUMN_NAME: SqlState = SqlState(Cow::Borrowed("HV007"));

    /// HV008
    pub const FDW_INVALID_COLUMN_NUMBER: SqlState = SqlState(Cow::Borrowed("HV008"));

    /// HV004
    pub const FDW_INVALID_DATA_TYPE: SqlState = SqlState(Cow::Borrowed("HV004"));

    /// HV006
    pub const FDW_INVALID_DATA_TYPE_DESCRIPTORS: SqlState = SqlState(Cow::Borrowed("HV006"));

    /// HV091
    pub const FDW_INVALID_DESCRIPTOR_FIELD_IDENTIFIER: SqlState = SqlState(Cow::Borrowed("HV091"));

    /// HV00B
    pub const FDW_INVALID_HANDLE: SqlState = SqlState(Cow::Borrowed("HV00B"));

    /// HV00C
    pub const FDW_INVALID_OPTION_INDEX: SqlState = SqlState(Cow::Borrowed("HV00C"));

    /// HV00D
    pub const FDW_INVALID_OPTION_NAME: SqlState = SqlState(Cow::Borrowed("HV00D"));

    /// HV090
    pub const FDW_INVALID_STRING_LENGTH_OR_BUFFER_LENGTH: SqlState =
        SqlState(Cow::Borrowed("HV090"));

    /// HV00A
    pub const FDW_INVALID_STRING_FORMAT: SqlState = SqlState(Cow::Borrowed("HV00A"));

    /// HV009
    pub const FDW_INVALID_USE_OF_NULL_POINTER: SqlState = SqlState(Cow::Borrowed("HV009"));

    /// HV014
    pub const FDW_TOO_MANY_HANDLES: SqlState = SqlState(Cow::Borrowed("HV014"));

    /// HV001
    pub const FDW_OUT_OF_MEMORY: SqlState = SqlState(Cow::Borrowed("HV001"));

    /// HV00P
    pub const FDW_NO_SCHEMAS: SqlState = SqlState(Cow::Borrowed("HV00P"));

    /// HV00J
    pub const FDW_OPTION_NAME_NOT_FOUND: SqlState = SqlState(Cow::Borrowed("HV00J"));

    /// HV00K
    pub const FDW_REPLY_HANDLE: SqlState = SqlState(Cow::Borrowed("HV00K"));

    /// HV00Q
    pub const FDW_SCHEMA_NOT_FOUND: SqlState = SqlState(Cow::Borrowed("HV00Q"));

    /// HV00R
    pub const FDW_TABLE_NOT_FOUND: SqlState = SqlState(Cow::Borrowed("HV00R"));

    /// HV00L
    pub const FDW_UNABLE_TO_CREATE_EXECUTION: SqlState = SqlState(Cow::Borrowed("HV00L"));

    /// HV00M
    pub const FDW_UNABLE_TO_CREATE_REPLY: SqlState = SqlState(Cow::Borrowed("HV00M"));

    /// HV00N
    pub const FDW_UNABLE_TO_ESTABLISH_CONNECTION: SqlState = SqlState(Cow::Borrowed("HV00N"));

    /// P0000
    pub const PLPGSQL_ERROR: SqlState = SqlState(Cow::Borrowed("P0000"));

    /// P0001
    pub const RAISE_EXCEPTION: SqlState = SqlState(Cow::Borrowed("P0001"));

    /// P0002
    pub const NO_DATA_FOUND: SqlState = SqlState(Cow::Borrowed("P0002"));

    /// P0003
    pub const TOO_MANY_ROWS: SqlState = SqlState(Cow::Borrowed("P0003"));

    /// P0004
    pub const ASSERT_FAILURE: SqlState = SqlState(Cow::Borrowed("P0004"));

    /// XX000
    pub const INTERNAL_ERROR: SqlState = SqlState(Cow::Borrowed("XX000"));

    /// XX001
    pub const DATA_CORRUPTED: SqlState = SqlState(Cow::Borrowed("XX001"));

    /// XX002
    pub const INDEX_CORRUPTED: SqlState = SqlState(Cow::Borrowed("XX002"));
}
#[cfg_attr(rustfmt, rustfmt_skip)]
static SQLSTATE_MAP: phf::Map<&'static str, SqlState> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (1, 16),
        (1, 97),
        (0, 146),
        (0, 0),
        (1, 0),
        (0, 217),
        (3, 134),
        (0, 2),
        (0, 6),
        (0, 32),
        (1, 99),
        (1, 227),
        (0, 6),
        (0, 163),
        (0, 89),
        (0, 5),
        (0, 3),
        (0, 200),
        (4, 99),
        (0, 32),
        (0, 19),
        (0, 82),
        (0, 54),
        (1, 126),
        (0, 11),
        (0, 83),
        (6, 102),
        (0, 67),
        (4, 162),
        (0, 13),
        (0, 116),
        (11, 57),
        (0, 210),
        (0, 4),
        (4, 127),
        (1, 133),
        (1, 158),
        (0, 180),
        (2, 201),
        (0, 148),
        (4, 135),
        (0, 5),
        (1, 1),
        (0, 0),
        (0, 191),
        (0, 171),
        (7, 38),
        (0, 0),
        (0, 0),
    ]),
    entries: ::phf::Slice::Static(&[
        ("22022", SqlState::INDICATOR_OVERFLOW),
        ("3B000", SqlState::SAVEPOINT_EXCEPTION),
        ("54023", SqlState::TOO_MANY_ARGUMENTS),
        ("25P02", SqlState::IN_FAILED_SQL_TRANSACTION),
        ("38002", SqlState::E_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED),
        ("02001", SqlState::NO_ADDITIONAL_DYNAMIC_RESULT_SETS_RETURNED),
        ("72000", SqlState::SNAPSHOT_TOO_OLD),
        ("42622", SqlState::NAME_TOO_LONG),
        ("42P19", SqlState::INVALID_RECURSION),
        ("F0000", SqlState::CONFIG_FILE_ERROR),
        ("HV014", SqlState::FDW_TOO_MANY_HANDLES),
        ("2BP01", SqlState::DEPENDENT_OBJECTS_STILL_EXIST),
        ("HV00C", SqlState::FDW_INVALID_OPTION_INDEX),
        ("01P01", SqlState::WARNING_DEPRECATED_FEATURE),
        ("03000", SqlState::SQL_STATEMENT_NOT_YET_COMPLETE),
        ("HV090", SqlState::FDW_INVALID_STRING_LENGTH_OR_BUFFER_LENGTH),
        ("3F000", SqlState::INVALID_SCHEMA_NAME),
        ("42000", SqlState::SYNTAX_ERROR_OR_ACCESS_RULE_VIOLATION),
        ("22P02", SqlState::INVALID_TEXT_REPRESENTATION),
        ("08000", SqlState::CONNECTION_EXCEPTION),
        ("38000", SqlState::EXTERNAL_ROUTINE_EXCEPTION),
        ("39001", SqlState::E_R_I_E_INVALID_SQLSTATE_RETURNED),
        ("HV009", SqlState::FDW_INVALID_USE_OF_NULL_POINTER),
        ("53200", SqlState::OUT_OF_MEMORY),
        ("22P01", SqlState::FLOATING_POINT_EXCEPTION),
        ("3D000", SqlState::INVALID_CATALOG_NAME),
        ("42702", SqlState::AMBIGUOUS_COLUMN),
        ("2201G", SqlState::INVALID_ARGUMENT_FOR_WIDTH_BUCKET_FUNCTION),
        ("08006", SqlState::CONNECTION_FAILURE),
        ("25003", SqlState::INAPPROPRIATE_ACCESS_MODE_FOR_BRANCH_TRANSACTION),
        ("P0004", SqlState::ASSERT_FAILURE),
        ("0B000", SqlState::INVALID_TRANSACTION_INITIATION),
        ("57014", SqlState::QUERY_CANCELED),
        ("57P01", SqlState::ADMIN_SHUTDOWN),
        ("22025", SqlState::INVALID_ESCAPE_SEQUENCE),
        ("55P04", SqlState::UNSAFE_NEW_ENUM_VALUE_USAGE),
        ("54001", SqlState::STATEMENT_TOO_COMPLEX),
        ("HV021", SqlState::FDW_INCONSISTENT_DESCRIPTOR_INFORMATION),
        ("0L000", SqlState::INVALID_GRANTOR),
        ("55P03", SqlState::LOCK_NOT_AVAILABLE),
        ("HV00P", SqlState::FDW_NO_SCHEMAS),
        ("0LP01", SqlState::INVALID_GRANT_OPERATION),
        ("40002", SqlState::T_R_INTEGRITY_CONSTRAINT_VIOLATION),
        ("2200B", SqlState::ESCAPE_CHARACTER_CONFLICT),
        ("42P02", SqlState::UNDEFINED_PARAMETER),
        ("2200D", SqlState::INVALID_ESCAPE_OCTET),
        ("25000", SqlState::INVALID_TRANSACTION_STATE),
        ("39P01", SqlState::E_R_I_E_TRIGGER_PROTOCOL_VIOLATED),
        ("42725", SqlState::AMBIGUOUS_FUNCTION),
        ("23000", SqlState::INTEGRITY_CONSTRAINT_VIOLATION),
        ("42P09", SqlState::AMBIGUOUS_ALIAS),
        ("42939", SqlState::RESERVED_NAME),
        ("57P02", SqlState::CRASH_SHUTDOWN),
        ("40P01", SqlState::T_R_DEADLOCK_DETECTED),
        ("HV00Q", SqlState::FDW_SCHEMA_NOT_FOUND),
        ("25P01", SqlState::NO_ACTIVE_SQL_TRANSACTION),
        ("55P02", SqlState::CANT_CHANGE_RUNTIME_PARAM),
        ("27000", SqlState::TRIGGERED_DATA_CHANGE_VIOLATION),
        ("XX002", SqlState::INDEX_CORRUPTED),
        ("0Z000", SqlState::DIAGNOSTICS_EXCEPTION),
        ("42P08", SqlState::AMBIGUOUS_PARAMETER),
        ("42602", SqlState::INVALID_NAME),
        ("55006", SqlState::OBJECT_IN_USE),
        ("44000", SqlState::WITH_CHECK_OPTION_VIOLATION),
        ("0F000", SqlState::LOCATOR_EXCEPTION),
        ("02000", SqlState::NO_DATA),
        ("22010", SqlState::INVALID_INDICATOR_PARAMETER_VALUE),
        ("24000", SqlState::INVALID_CURSOR_STATE),
        ("P0002", SqlState::NO_DATA_FOUND),
        ("P0003", SqlState::TOO_MANY_ROWS),
        ("2201W", SqlState::INVALID_ROW_COUNT_IN_LIMIT_CLAUSE),
        ("2200H", SqlState::SEQUENCE_GENERATOR_LIMIT_EXCEEDED),
        ("42803", SqlState::GROUPING_ERROR),
        ("HV00D", SqlState::FDW_INVALID_OPTION_NAME),
        ("HV008", SqlState::FDW_INVALID_COLUMN_NUMBER),
        ("HV00M", SqlState::FDW_UNABLE_TO_CREATE_REPLY),
        ("2200M", SqlState::INVALID_XML_DOCUMENT),
        ("HV00R", SqlState::FDW_TABLE_NOT_FOUND),
        ("25001", SqlState::ACTIVE_SQL_TRANSACTION),
        ("42P05", SqlState::DUPLICATE_PSTATEMENT),
        ("2200G", SqlState::MOST_SPECIFIC_TYPE_MISMATCH),
        ("0Z002", SqlState::STACKED_DIAGNOSTICS_ACCESSED_WITHOUT_ACTIVE_HANDLER),
        ("40003", SqlState::T_R_STATEMENT_COMPLETION_UNKNOWN),
        ("08004", SqlState::SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION),
        ("55000", SqlState::OBJECT_NOT_IN_PREREQUISITE_STATE),
        ("42883", SqlState::UNDEFINED_FUNCTION),
        ("23P01", SqlState::EXCLUSION_VIOLATION),
        ("F0001", SqlState::LOCK_FILE_EXISTS),
        ("0A000", SqlState::FEATURE_NOT_SUPPORTED),
        ("2200C", SqlState::INVALID_USE_OF_ESCAPE_CHARACTER),
        ("42846", SqlState::CANNOT_COERCE),
        ("HV091", SqlState::FDW_INVALID_DESCRIPTOR_FIELD_IDENTIFIER),
        ("22015", SqlState::INTERVAL_FIELD_OVERFLOW),
        ("53100", SqlState::DISK_FULL),
        ("39000", SqlState::EXTERNAL_ROUTINE_INVOCATION_EXCEPTION),
        ("HV00A", SqlState::FDW_INVALID_STRING_FORMAT),
        ("58P02", SqlState::DUPLICATE_FILE),
        ("P0001", SqlState::RAISE_EXCEPTION),
        ("39004", SqlState::E_R_I_E_NULL_VALUE_NOT_ALLOWED),
        ("22P04", SqlState::BAD_COPY_FILE_FORMAT),
        ("39P03", SqlState::E_R_I_E_EVENT_TRIGGER_PROTOCOL_VIOLATED),
        ("25002", SqlState::BRANCH_TRANSACTION_ALREADY_ACTIVE),
        ("22024", SqlState::UNTERMINATED_C_STRING),
        ("42601", SqlState::SYNTAX_ERROR),
        ("HV024", SqlState::FDW_INVALID_ATTRIBUTE_VALUE),
        ("26000", SqlState::INVALID_SQL_STATEMENT_NAME),
        ("2202E", SqlState::ARRAY_ELEMENT_ERROR),
        ("2200T", SqlState::INVALID_XML_PROCESSING_INSTRUCTION),
        ("58000", SqlState::SYSTEM_ERROR),
        ("42501", SqlState::INSUFFICIENT_PRIVILEGE),
        ("22002", SqlState::NULL_VALUE_NO_INDICATOR_PARAMETER),
        ("23514", SqlState::CHECK_VIOLATION),
        ("42809", SqlState::WRONG_OBJECT_TYPE),
        ("20000", SqlState::CASE_NOT_FOUND),
        ("22001", SqlState::STRING_DATA_RIGHT_TRUNCATION),
        ("HV00K", SqlState::FDW_REPLY_HANDLE),
        ("08007", SqlState::TRANSACTION_RESOLUTION_UNKNOWN),
        ("42830", SqlState::INVALID_FOREIGN_KEY),
        ("2201F", SqlState::INVALID_ARGUMENT_FOR_POWER_FUNCTION),
        ("2D000", SqlState::INVALID_TRANSACTION_TERMINATION),
        ("38001", SqlState::E_R_E_CONTAINING_SQL_NOT_PERMITTED),
        ("53000", SqlState::INSUFFICIENT_RESOURCES),
        ("XX001", SqlState::DATA_CORRUPTED),
        ("54011", SqlState::TOO_MANY_COLUMNS),
        ("57P04", SqlState::DATABASE_DROPPED),
        ("2F005", SqlState::S_R_E_FUNCTION_EXECUTED_NO_RETURN_STATEMENT),
        ("HV00N", SqlState::FDW_UNABLE_TO_ESTABLISH_CONNECTION),
        ("25004", SqlState::INAPPROPRIATE_ISOLATION_LEVEL_FOR_BRANCH_TRANSACTION),
        ("42P11", SqlState::INVALID_CURSOR_DEFINITION),
        ("42701", SqlState::DUPLICATE_COLUMN),
        ("42P18", SqlState::INDETERMINATE_DATATYPE),
        ("08001", SqlState::SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION),
        ("01007", SqlState::WARNING_PRIVILEGE_NOT_GRANTED),
        ("0100C", SqlState::WARNING_DYNAMIC_RESULT_SETS_RETURNED),
        ("2200L", SqlState::NOT_AN_XML_DOCUMENT),
        ("22011", SqlState::SUBSTRING_ERROR),
        ("42P20", SqlState::WINDOWING_ERROR),
        ("42P12", SqlState::INVALID_DATABASE_DEFINITION),
        ("22023", SqlState::INVALID_PARAMETER_VALUE),
        ("22016", SqlState::INVALID_ARGUMENT_FOR_NTH_VALUE),
        ("53400", SqlState::CONFIGURATION_LIMIT_EXCEEDED),
        ("23001", SqlState::RESTRICT_VIOLATION),
        ("428C9", SqlState::GENERATED_ALWAYS),
        ("42723", SqlState::DUPLICATE_FUNCTION),
        ("HV007", SqlState::FDW_INVALID_COLUMN_NAME),
        ("38003", SqlState::E_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED),
        ("40001", SqlState::T_R_SERIALIZATION_FAILURE),
        ("42P07", SqlState::DUPLICATE_TABLE),
        ("22021", SqlState::CHARACTER_NOT_IN_REPERTOIRE),
        ("08P01", SqlState::PROTOCOL_VIOLATION),
        ("39P02", SqlState::E_R_I_E_SRF_PROTOCOL_VIOLATED),
        ("22P03", SqlState::INVALID_BINARY_REPRESENTATION),
        ("XX000", SqlState::INTERNAL_ERROR),
        ("42P17", SqlState::INVALID_OBJECT_DEFINITION),
        ("2200N", SqlState::INVALID_XML_CONTENT),
        ("23502", SqlState::NOT_NULL_VIOLATION),
        ("HV00B", SqlState::FDW_INVALID_HANDLE),
        ("28000", SqlState::INVALID_AUTHORIZATION_SPECIFICATION),
        ("2201E", SqlState::INVALID_ARGUMENT_FOR_LOG),
        ("22018", SqlState::INVALID_CHARACTER_VALUE_FOR_CAST),
        ("HV004", SqlState::FDW_INVALID_DATA_TYPE),
        ("2F000", SqlState::SQL_ROUTINE_EXCEPTION),
        ("0P000", SqlState::INVALID_ROLE_SPECIFICATION),
        ("42P04", SqlState::DUPLICATE_DATABASE),
        ("42P06", SqlState::DUPLICATE_SCHEMA),
        ("28P01", SqlState::INVALID_PASSWORD),
        ("2F004", SqlState::S_R_E_READING_SQL_DATA_NOT_PERMITTED),
        ("57000", SqlState::OPERATOR_INTERVENTION),
        ("P0000", SqlState::PLPGSQL_ERROR),
        ("42712", SqlState::DUPLICATE_ALIAS),
        ("22019", SqlState::INVALID_ESCAPE_CHARACTER),
        ("22012", SqlState::DIVISION_BY_ZERO),
        ("25007", SqlState::SCHEMA_AND_DATA_STATEMENT_MIXING_NOT_SUPPORTED),
        ("22026", SqlState::STRING_DATA_LENGTH_MISMATCH),
        ("0F001", SqlState::L_E_INVALID_SPECIFICATION),
        ("22009", SqlState::INVALID_TIME_ZONE_DISPLACEMENT_VALUE),
        ("42804", SqlState::DATATYPE_MISMATCH),
        ("23503", SqlState::FOREIGN_KEY_VIOLATION),
        ("2201B", SqlState::INVALID_REGULAR_EXPRESSION),
        ("2B000", SqlState::DEPENDENT_PRIVILEGE_DESCRIPTORS_STILL_EXIST),
        ("2200S", SqlState::INVALID_XML_COMMENT),
        ("22003", SqlState::NUMERIC_VALUE_OUT_OF_RANGE),
        ("22P06", SqlState::NONSTANDARD_USE_OF_ESCAPE_CHARACTER),
        ("3B001", SqlState::S_E_INVALID_SPECIFICATION),
        ("2202G", SqlState::INVALID_TABLESAMPLE_REPEAT),
        ("21000", SqlState::CARDINALITY_VIOLATION),
        ("25005", SqlState::NO_ACTIVE_SQL_TRANSACTION_FOR_BRANCH_TRANSACTION),
        ("08003", SqlState::CONNECTION_DOES_NOT_EXIST),
        ("42710", SqlState::DUPLICATE_OBJECT),
        ("2F003", SqlState::S_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED),
        ("42611", SqlState::INVALID_COLUMN_DEFINITION),
        ("25006", SqlState::READ_ONLY_SQL_TRANSACTION),
        ("2F002", SqlState::S_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED),
        ("HV00J", SqlState::FDW_OPTION_NAME_NOT_FOUND),
        ("38004", SqlState::E_R_E_READING_SQL_DATA_NOT_PERMITTED),
        ("42P16", SqlState::INVALID_TABLE_DEFINITION),
        ("25008", SqlState::HELD_CURSOR_REQUIRES_SAME_ISOLATION_LEVEL),
        ("22007", SqlState::INVALID_DATETIME_FORMAT),
        ("22005", SqlState::ERROR_IN_ASSIGNMENT),
        ("HV001", SqlState::FDW_OUT_OF_MEMORY),
        ("HV000", SqlState::FDW_ERROR),
        ("42P10", SqlState::INVALID_COLUMN_REFERENCE),
        ("54000", SqlState::PROGRAM_LIMIT_EXCEEDED),
        ("23505", SqlState::UNIQUE_VIOLATION),
        ("42703", SqlState::UNDEFINED_COLUMN),
        ("53300", SqlState::TOO_MANY_CONNECTIONS),
        ("HV00L", SqlState::FDW_UNABLE_TO_CREATE_EXECUTION),
        ("01000", SqlState::WARNING),
        ("42P01", SqlState::UNDEFINED_TABLE),
        ("22027", SqlState::TRIM_ERROR),
        ("00000", SqlState::SUCCESSFUL_COMPLETION),
        ("22008", SqlState::DATETIME_FIELD_OVERFLOW),
        ("42P21", SqlState::COLLATION_MISMATCH),
        ("HV010", SqlState::FDW_FUNCTION_SEQUENCE_ERROR),
        ("2201X", SqlState::INVALID_ROW_COUNT_IN_RESULT_OFFSET_CLAUSE),
        ("HV006", SqlState::FDW_INVALID_DATA_TYPE_DESCRIPTORS),
        ("2200F", SqlState::ZERO_LENGTH_CHARACTER_STRING),
        ("22P05", SqlState::UNTRANSLATABLE_CHARACTER),
        ("42P13", SqlState::INVALID_FUNCTION_DEFINITION),
        ("40000", SqlState::TRANSACTION_ROLLBACK),
        ("01008", SqlState::WARNING_IMPLICIT_ZERO_BIT_PADDING),
        ("58P01", SqlState::UNDEFINED_FILE),
        ("HV002", SqlState::FDW_DYNAMIC_PARAMETER_VALUE_NEEDED),
        ("42704", SqlState::UNDEFINED_OBJECT),
        ("HV005", SqlState::FDW_COLUMN_NAME_NOT_FOUND),
        ("42P14", SqlState::INVALID_PSTATEMENT_DEFINITION),
        ("42P22", SqlState::INDETERMINATE_COLLATION),
        ("01004", SqlState::WARNING_STRING_DATA_RIGHT_TRUNCATION),
        ("34000", SqlState::INVALID_CURSOR_NAME),
        ("09000", SqlState::TRIGGERED_ACTION_EXCEPTION),
        ("22014", SqlState::INVALID_ARGUMENT_FOR_NTILE),
        ("01006", SqlState::WARNING_PRIVILEGE_NOT_REVOKED),
        ("57P03", SqlState::CANNOT_CONNECT_NOW),
        ("22004", SqlState::NULL_VALUE_NOT_ALLOWED),
        ("2202H", SqlState::INVALID_TABLESAMPLE_ARGUMENT),
        ("42P15", SqlState::INVALID_SCHEMA_DEFINITION),
        ("22000", SqlState::DATA_EXCEPTION),
        ("58030", SqlState::IO_ERROR),
        ("25P03", SqlState::IDLE_IN_TRANSACTION_SESSION_TIMEOUT),
        ("42P03", SqlState::DUPLICATE_CURSOR),
        ("01003", SqlState::WARNING_NULL_VALUE_ELIMINATED_IN_SET_FUNCTION),
    ]),
};
