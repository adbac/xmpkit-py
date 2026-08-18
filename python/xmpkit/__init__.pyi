"""Python bindings for xmpkit (Rust)"""

from collections.abc import Sequence
from typing import Any, Final, TypeAlias, final

from _typeshed import Incomplete

XmpValueType: TypeAlias = (
    XmpValue.Array
    | XmpValue.Boolean
    | XmpValue.DateTime
    | XmpValue.Integer
    | XmpValue.String
    | XmpValue.Structure
)

class XmpError(Exception):
    """Error type for XMP operations"""

@final
class XmpDateTime:
    """XMP Date/Time structure

    Represents a date/time value with optional components.
    XMP supports partial dates (e.g., just year, or year-month).
    """
    def __new__(
        cls,
        /,
        year: int = 0,
        month: int = 0,
        day: int = 0,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        nanosecond: int = 0,
        has_date: bool = False,
        has_time: bool = False,
        has_timezone: bool = False,
        tz_sign: int = 0,
        tz_hour: int = 0,
        tz_minute: int = 0,
    ) -> XmpDateTime:
        """Create a new XMP date/time from the provided values"""
    @classmethod
    def parse(cls, /, s: str) -> XmpDateTime:
        """Parse an XMP date/time string

        Args:
            s: The date/time string to parse

        Raises:
            XmpError: If the string could not be parsed

        XMP date/time format:

        - `YYYY` - year only
        - `YYYY-MM` - year and month
        - `YYYY-MM-DD` - date only
        - `YYYY-MM-DDThh:mm:ss` - date and time
        - `YYYY-MM-DDThh:mm:ss.sss` - with fractional seconds
        - `YYYY-MM-DDThh:mm:ssZ` - UTC timezone
        - `YYYY-MM-DDThh:mm:ss+hh:mm` - timezone offset
        - `YYYY-MM-DDThh:mm:ss-hh:mm` - negative timezone offset

        Examples::

            from xmpkit import XmpDateTime

            dt = XmpDateTime.parse("2023-12-25T10:30:00Z")
            assert dt.year == 2023
            assert dt.month == 12
            assert dt.day == 25
        """
    def format(self, /) -> str:
        """Format an XMP date/time to string

        Formats the date/time according to XMP specification:

        - Year only: `YYYY`
        - Year and month: `YYYY-MM`
        - Date only: `YYYY-MM-DD`
        - Date and time: `YYYY-MM-DDThh:mm:ss`
        - With fractional seconds: `YYYY-MM-DDThh:mm:ss.sss`
        - With timezone: `YYYY-MM-DDThh:mm:ssZ` or `YYYY-MM-DDThh:mm:ss+hh:mm`
        """
    def validate(self, /) -> None:
        """Validate the date/time values

        Checks that all values are within valid ranges.

        Raises:
            XmpError: If the date/time data is not valid
        """

    # Fields
    @property
    def day(self, /) -> int:
        """Day (1-31, 0 means not set)"""
    @day.setter
    def day(self, /, value: int) -> None: ...
    @property
    def has_date(self, /) -> bool:
        """Whether date components are present"""
    @has_date.setter
    def has_date(self, /, value: bool) -> None: ...
    @property
    def has_time(self, /) -> bool:
        """Whether time components are present"""
    @has_time.setter
    def has_time(self, /, value: bool) -> None: ...
    @property
    def has_timezone(self, /) -> bool:
        """Whether timezone is present"""
    @has_timezone.setter
    def has_timezone(self, /, value: bool) -> None: ...
    @property
    def hour(self, /) -> int:
        """Hour (0-23)"""
    @hour.setter
    def hour(self, /, value: int) -> None: ...
    @property
    def minute(self, /) -> int:
        """Minute (0-59)"""
    @minute.setter
    def minute(self, /, value: int) -> None: ...
    @property
    def month(self, /) -> int:
        """Month (1-12, 0 means not set)"""
    @month.setter
    def month(self, /, value: int) -> None: ...
    @property
    def nanosecond(self, /) -> int:
        """Nanoseconds (0-999999999)"""
    @nanosecond.setter
    def nanosecond(self, /, value: int) -> None: ...
    @property
    def second(self, /) -> int:
        """Second (0-59)"""
    @second.setter
    def second(self, /, value: int) -> None: ...
    @property
    def tz_hour(self, /) -> int:
        """Timezone hour offset (0-23)"""
    @tz_hour.setter
    def tz_hour(self, /, value: int) -> None: ...
    @property
    def tz_minute(self, /) -> int:
        """Timezone minute offset (0-59)"""
    @tz_minute.setter
    def tz_minute(self, /, value: int) -> None: ...
    @property
    def tz_sign(self, /) -> int:
        """Timezone sign: -1 (west), 0 (UTC), +1 (east)"""
    @tz_sign.setter
    def tz_sign(self, /, value: int) -> None: ...
    @property
    def year(self, /) -> int:
        """Year (can be negative for BCE dates)"""
    @year.setter
    def year(self, /, value: int) -> None: ...

@final
class XmpFile:
    """High-level API for working with XMP metadata in files

    # File Update Behavior

    When a file is opened with [`XmpOptions.with_for_update`][], changes made via
    [`put_xmp`][] are not written to disk immediately. The file remains open
    and changes are only written when [`close`][] or [`try_close`][] is called.

    Examples::

        from xmpkit import XmpFile, XmpOptions, XmpMeta, XmpValue

        file = XmpFile()
        file.open_with("image.jpg", XmpOptions().for_update())

        if (meta := file.get_xmp()) is not None:
            meta.set_property(
                "http://ns.adobe.com/xap/1.0/",
                "CreatorTool",
                XmpValue.String("MyApp"),
            )
            file.put_xmp(meta)

        # Changes are written to disk when try_close() is called
        file.try_close()
    """
    def __new__(cls, /) -> XmpFile:
        """Create a new empty XMP file object

        Use `open*` or `from_*()` methods to load metadata from a file.
        """
    def open(self, /, path: str) -> None:
        """Open a file from a `path`

        Examples::

            from xmpkit import XmpFile

            file = XmpFile()
            file.open("image.jpg")
        """
    def open_with(self, /, path: str, options: XmpOptions) -> None:
        """Open a file from a path with `options`

        Args:
            path: The path to the file with the metadata
            options: The XMP options to open the file with

        Examples::

            from xmpkit import XmpFile, XmpOptions

            file = XmpFile()
            file.open_with("image.jpg", XmpOptions().for_update())
        """
    def from_bytes(self, /, data: bytes) -> None:
        """Open a file from bytes represented by `data`

        Examples::

            from xmpkit import XmpFile

            jpeg_data: bytes = ...  # your JPEG file data

            file = XmpFile()
            file.from_bytes(jpeg_data)
        """
    def from_bytes_with(self, /, data: bytes, options: XmpOptions) -> None:
        """Open a file from bytes with options

        This method allows you to specify opening options, such as forcing packet scanning
        or requiring a smart handler.

        Args:
            data: The bytes representing the file to open
            option: The XMP options to open the file with

        Examples::

            from xmpkit import XmpFile, XmpOptions

            data: bytes = ...  # your file data

            file = XmpFile()
            file.from_bytes_with(data, XmpOptions().with_use_packet_scanning())
        """
    def get_xmp(self, /) -> XmpMeta | None:
        """Get the XMP metadata

        Returns:
            The metadata object if loaded or found, `None` otherwise.
        """
    def put_xmp(self, /, meta: XmpMeta) -> None:
        """Put XMP metadata represented by `meta`

        Replaces any existing metadata.

        # Update Behavior

        - If the file was opened with [`XmpOptions.with_for_update`][], changes are
          not written to disk immediately. Call [`close`][] or [`try_close`][] to
          write changes to disk.
        - If the file was opened read-only, this only updates the in-memory metadata.

        Examples::

            from xmpkit import XmpFile, XmpOptions, XmpMeta, XmpValue

            file = XmpFile()
            file.open_with("image.jpg", XmpOptions().with_for_update())

            meta = file.get_xmp() or XmpMeta();
            meta.set_property(
                "http://ns.adobe.com/xap/1.0/",
                "CreatorTool",
                XmpValue.String("MyApp"),
            )

            file.put_xmp(meta)
            # Write changes to disk
            file.try_close()
        """
    def write_to_bytes(self, /) -> bytes:
        """Write XMP metadata to bytes

        Examples::

            from xmpkit import XmpFile

            input_data: bytes = ...  # your JPEG file data

            file = XmpFile()
            file.from_bytes(input_data)
            # ... modify metadata ...
            output_data = file.write_to_bytes()
        """
    def save(self, /, path: str) -> None:
        """Write XMP metadata to a file `path`

        Examples::

            from xmpkit import XmpFile, XmpMeta

            file = XmpFile()
            file.open("image.jpg")
            # ... modify metadata ...
            file.save("output.jpg")
        """
    def close(self, /) -> None:
        """Explicitly closes an opened file.

        Performs any necessary output to the file and closes it. Files that are
        opened for update are written to only when closing. If the file is opened
        for read-only access (using [`XmpOptions.with_for_read()`][]), the disk
        file is closed immediately after reading the data from it; the `XmpFile`
        object, however, remains in the open state. You must call [`close`][]
        when finished using it.

        # Errors

        This method ignores errors for backward compatibility. If you want to
        handle errors, use [`try_close`][] instead.

        Examples::

            use xmpkit::{XmpFile, XmpOptions};
            # fn main() -> Result<(), Box<dyn std::error::Error>> {
            let mut file = XmpFile::new();
            file.open_with("image.jpg", XmpOptions::default().for_update())?;
            // ... modify metadata ...
            file.close(); // Ignores errors
        """
    def try_close(self, /) -> None:
        """Explicitly closes an opened file with error handling.

        Performs any necessary output to the file and closes it. Files that are
        opened for update are written to only when closing. If the file is opened
        for read-only access (using [`XmpOptions.with_for_read`][]), the disk file
        is closed immediately after reading the data from it; the `XmpFile` object,
        however, remains in the open state. You must call [`try_close`][] when
        finished using it.

        Raises:
            XmpError: If writing the file fails.

        Examples::

            from xmpkit import XmpFile, XmpOptions

            file = XmpFile()
            file.open_with("image.jpg", XmpOptions().with_for_update())
            # ... modify metadata ...
            file.try_close()  # Raises error if write fails
        """
    @staticmethod
    def scan_for_xmp_packet(file_data: bytes) -> XmpMeta | None:
        """Scan file content for XMP packet (packet scanning mode)

        This method searches for XMP packets in file content by looking for
        the `<?xpacket` marker. Used when packet scanning is requested.

        Args:
            file_data: The file bytes to scan
        """

@final
class XmpMeta:
    """Main structure for working with XMP metadata"""
    def __new__(cls, /) -> XmpMeta:
        """Create a new empty XMP metadata object"""
    @property
    def about_uri(self, /) -> str | None:
        """The about URI"""
    @about_uri.setter
    def about_uri(self, /, uri: str) -> None: ...
    def all_properties(self, /) -> list[XmpProperty]:
        """Returns all top-level properties in this metadata object."""
    def append_array_item(
        self, /, namespace: str, path: str, value: XmpValueType
    ) -> None:
        """Append an item to an array property

        Args:
            namespace: The namespace URI or prefix
            path: The array property path
            value: The value to append
        """
    def delete_array_item(self, /, namespace: str, path: str, index: int) -> None:
        """Delete an item from an array property

        Args:
            namespace: The namespace URI or prefix
            path: The array property path
            index: The index to delete (0-based)
        """
    def delete_property(self, /, namespace: str, path: str) -> None:
        """Delete a property

        Args:
            namespace: The namespace URI or prefix
            path: The property path
        """
    def delete_struct_field(
        self, /, namespace: str, struct_path: str, field_name: str
    ) -> None:
        """Delete a structure field

        Args:
            namespace: The namespace URI or prefix
            struct_path: The structure property path
            field_name: The field name to delete
        """
    def get_array_item(
        self, /, namespace: str, path: str, index: int
    ) -> XmpValueType | None:
        """Get an array item by index

        Args:
            namespace: The namespace URI or prefix
            path: The array property path (e.g., "creator")
            index: The array index (0-based)
        """
    def get_array_size(self, /, namespace: str, path: str) -> int | None:
        """Get the size of an array property

        Args:
            namespace: The namespace URI or prefix
            path: The array property path
        """
    def get_date_time(self, /, namespace: str, path: str) -> XmpDateTime | None:
        """Get a date/time property

        This is a convenience method that parses a date/time property value
        and returns it as an [`XmpDateTime`][].

        Args:
            namespace: The namespace URI or prefix
            path: The property path

        Returns:
            An XMP date/time object if the property exists and can be parsed,
                `None` otherwise.

        Examples::

            from xmpkit import XmpMeta, XmpValue, XmpDateTime

            meta = XmpMeta()
            meta.set_property(
                "http://ns.adobe.com/xap/1.0/",
                "ModifyDate",
                XmpValue.DateTime("2023-12-25T10:30:00Z")
            )

            dt = meta.get_date_time("http://ns.adobe.com/xap/1.0/", "ModifyDate")
            assert dt.year = 2023
            assert dt.month = 12
            assert dt.day = 25
        """
    def get_localized_text(
        self, /, namespace: str, property: str, generic_lang: str, specific_lang: str
    ) -> tuple[str, str] | None:
        """Get a localized text property

        This method searches for a localized text value matching the specified
        language codes. It follows XMP language matching rules:

        1. Exact match for specific_lang
        2. Match for generic_lang if specific_lang not found
        3. Fallback to "x-default" if neither found

        Args:
            namespace: The namespace URI or prefix
            property: The property name
            generic_lang: Generic language code (e.g., "en"), can be empty string
            specific_lang: Specific language code (e.g., "en-US"), required

        Returns:
            If the localized text if found, a tuple holding the text value and the actual
                language code used (may differ from requested).
                `None` if the property doesn't exist or no matching language found.

        Examples::

            from xmpkit import XmpMeta

            meta = XmpMeta()
            meta.set_localized_text(
                "http://purl.org/dc/elements/1.1/",
                "title",
                "",
                "x-default",
                "Default Title"
            )
            value, lang = meta.get_localized_text(
                "http://purl.org/dc/elements/1.1/",
                "title",
                "",
                "x-default"
            )
            assert value == "Default Title"
            assert lang == "x-default"
        """
    def get_property(self, /, namespace: str, path: str) -> XmpValueType | None:
        """Get a property value. It will return an [`XmpValue.Array`][], [`XmpValue.Structure`][] or an [`XmpValue.String`][].

        Args:
            namespace: The namespace URI or prefix
            path: The property path (e.g., "CreatorTool" or "creator[1]")
        """
    def get_struct_field(
        self, /, namespace: str, struct_path: str, field_name: str
    ) -> XmpValueType | None:
        """Get a structure field value

        Args:
            namespace: The namespace URI or prefix
            struct_path: The structure property path
            field_name: The field name within the structure
        """
    def has_property(self, /, namespace: str, path: str) -> bool:
        """Check if a property exists

        Args:
            namespace: The namespace URI or prefix
            path: The property path
        """
    def insert_array_item(
        self, /, namespace: str, path: str, index: int, value: XmpValueType
    ) -> None:
        """Insert an item into an array property at a specific index

        Args:
            namespace: The namespace URI or prefix
            path: The array property path
            index: The index to insert at (0-based)
            value: The value to insert
        """
    @classmethod
    def parse(cls, /, s: str) -> XmpMeta:
        """Parse XMP metadata from a string

        The string should contain a complete XMP Packet (with or without
        the `<?xpacket>` wrapper).

        Args:
            s: The string to parse the metadata from
        """
    def serialize(self, /) -> str:
        """Serialize to RDF/XML string"""
    def serialize_packet(self, /) -> str:
        """Serialize to XMP Packet format"""
    def serialize_packet_with_padding(self, /, target_length: int) -> str:
        """Serialize to XMP Packet format with padding to reach a target length

        This is useful for in-place updates where the new packet needs to fit
        within the space of an existing packet.

        Args:
            target_length: The desired total packet length in bytes

        Raises:
            XmpError: If the serialized packet exceeds target_length

        Returns:
            The serialized packet with padding
        """
    def set_date_time(self, /, namespace: str, path: str, dt: XmpDateTime) -> None:
        """Set a date/time property

        This is a convenience method that validates and formats the date/time value
        before setting it as a property.

        Args:
            namespace: The namespace URI or prefix
            path: The property path
            dt: The date/time value

        Examples::

            from xmpkit import XmpMeta, XmpDateTime

            meta = XmpMeta()

            dt = XmpDateTime()

            dt.has_date = true
            dt.has_time = true
            dt.year = 2023
            dt.month = 12
            dt.day = 25
            dt.hour = 10
            dt.minute = 30
            dt.second = 0
            dt.has_timezone = true
            dt.tz_sign = 0  # UTC

            meta.set_date_time("http://ns.adobe.com/xap/1.0/", "ModifyDate", dt)
        """
    def set_localized_text(
        self,
        /,
        namespace: str,
        property: str,
        _generic_lang: str,
        specific_lang: str,
        value: str,
    ) -> None:
        """Set a localized text property

        Localized text properties are stored as `rdf:Alt` arrays, where each item
        has an `xml:lang` qualifier indicating its language.

        Args:
            namespace: The namespace URI or prefix
            property: The property name
            generic_lang: Generic language code (e.g., "en"), can be empty string
            specific_lang: Specific language code (e.g., "en-US"), required
            value: The text value to set

        Examples::

            from xmpkit import XmpMeta, XmpValue

            meta = XmpMeta()
            meta.set_localized_text(
                "http://purl.org/dc/elements/1.1/",
                "title",
                "",
                "x-default",
                "Default Title"
            )
        """
    def set_property(self, /, namespace: str, path: str, value: XmpValueType) -> None:
        """Set a property value

        Args:
            namespace: The namespace URI or prefix
            path: The property path
            value: The value to set
        """
    def set_struct_field(
        self, /, namespace: str, struct_path: str, field_name: str, value: XmpValueType
    ) -> None:
        """Set a structure field value

        Args:
            namespace: The namespace URI or prefix
            struct_path: The structure property path
            field_name: The field name within the structure
            value: The value to set
        """

@final
class XmpOptions:
    """Options for XMP file operations.

    Use the builder pattern to configure options. These options control how
    file handlers read and process XMP metadata.

    Examples::

        from xmpkit import XmpFile, XmpOptions

        file = XmpFile()
        # Open for update with strict mode
        file.open_with("photo.jpg", XmpOptions().with_for_update().with_strict())
        # ... modify metadata ...
        file.try_close()
    """
    def __new__(
        cls,
        /,
        for_update: bool = False,
        only_xmp: bool = False,
        force_given_handler: bool = False,
        strict: bool = False,
        use_smart_handler: bool = False,
        use_packet_scanning: bool = False,
        limited_scanning: bool = False,
    ) -> XmpOptions:
        """Create a new XMP options object from the provided values"""
    @property
    def for_update(self, /) -> bool:
        """Open for reading and writing (default: read-only)"""
    @for_update.setter
    def for_update(self, /, value: bool) -> None: ...
    @property
    def force_given_handler(self, /) -> bool:
        """Force use of the given handler (format)"""
    @force_given_handler.setter
    def force_given_handler(self, /, value: bool) -> None: ...
    @property
    def limited_scanning(self, /) -> bool:
        """Only packet scan files "known" to need scanning"""
    @limited_scanning.setter
    def limited_scanning(self, /, value: bool) -> None: ...
    @property
    def only_xmp(self, /) -> bool:
        """Only the XMP is wanted, skip reconciliation with native metadata"""
    @only_xmp.setter
    def only_xmp(self, /, value: bool) -> None: ...
    @property
    def strict(self, /) -> bool:
        """Be strict about only attempting to use the designated file handler"""
    @strict.setter
    def strict(self, /, value: bool) -> None: ...
    @property
    def use_packet_scanning(self, /) -> bool:
        """Force packet scanning (do not use smart handler)"""
    @use_packet_scanning.setter
    def use_packet_scanning(self, /, value: bool) -> None: ...
    @property
    def use_smart_handler(self, /) -> bool:
        """Require the use of a smart handler"""
    @use_smart_handler.setter
    def use_smart_handler(self, /, value: bool) -> None: ...
    def with_for_read(self, /) -> XmpOptions:
        """Open for read-only access (default)."""
    def with_for_update(self, /) -> XmpOptions:
        """Open for reading and writing.

        Files opened for update are written to only when closing.
        """
    def with_force_given_handler(self, /) -> XmpOptions:
        """Force use of the given handler (format).

        Do not even verify the format.
        """
    def with_limited_scanning(self, /) -> XmpOptions:
        """Only packet scan files "known" to need scanning."""
    def with_only_xmp(self, /) -> XmpOptions:
        """Only the XMP is wanted.

        This allows space/time optimizations by skipping reconciliation
        with native metadata formats (e.g., QuickTime metadata in MPEG4).
        """
    def with_strict(self, /) -> XmpOptions:
        """Be strict about only attempting to use the designated file handler.

        Do not fall back to other handlers.
        """
    def with_use_packet_scanning(self, /) -> XmpOptions:
        """Force packet scanning.

        Do not use a smart handler.
        """
    def with_use_smart_handler(self, /) -> XmpOptions:
        """Require the use of a smart handler.

        Do not fall back to packet scanning.
        """

@final
class XmpProperty:
    """A property entry produced by iterating an [`XmpMeta`][] instance."""
    def __new__(
        cls, /, namespace_uri: str, name: str, value: XmpValueType
    ) -> XmpProperty:
        """Create a new XMP property object from the provided values"""
    @property
    def name(self, /) -> str:
        """Property name (e.g., "CreatorTool", "creator", "Flash")"""
    @property
    def namespace_uri(self, /) -> str:
        """Expanded namespace URI for the property (e.g., `http://ns.adobe.com/xap/1.0/`)"""
    @property
    def value(self, /) -> XmpValueType:
        """Property value"""

class XmpValue:
    """XMP value types

    This classes defines subclasses for the value types that can be stored in XMP properties.

    All child classes inherit from this class, but the documentation doesn't allow for this
    kind of recursive inheritance, so the child classes are typed this way.
    The typing is still valid but just doesn't represent the full structure of the objects
    """

    def as_array(self, /) -> list[XmpValueType] | None:
        """Get the value as a list, if it is an [`Array`][]."""
    def as_bool(self, /) -> bool | None:
        """Get the value as a boolean, if it is a boolean type, or can be converted to."""
    def as_int(self, /) -> int | None:
        """Get the value as an integer, if it is an integer type, or can be converted to."""
    def as_str(self, /) -> str | None:
        """Get the value as a string, if it is a string or date/time type"""
    def as_structure(self, /) -> dict[str, XmpValueType] | None:
        """Get the value as a dict, if it is a [`Structure`][]."""

    @final
    class Array:
        """Array of values"""

        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> list[XmpValueType]: ...
        def __getitem__(self, key: int, /) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: Sequence[XmpValueType]) -> XmpValue.Array: ...
        def as_array(self, /) -> list[XmpValueType]:
            """Get the value as a list."""

    @final
    class Boolean:
        """Boolean value"""

        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> bool: ...
        def __getitem__(self, key: int, /) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: bool) -> XmpValue.Boolean: ...
        def as_bool(self, /) -> bool:
            """Get the value as a boolean."""

    @final
    class DateTime:
        """Date/time value (ISO 8601 format)"""

        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> str: ...
        def __getitem__(self, key: int, /) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: str) -> XmpValue.DateTime: ...
        def as_str(self, /) -> str:
            """Get the value as a string.

            !!! version-added "Added in version 0.1.2"
            """

    @final
    class Integer:
        """Integer value"""

        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> int: ...
        def __getitem__(self, key: int, /) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: int) -> XmpValue.Integer: ...
        def as_int(self, /) -> int:
            """Get the value as an integer."""

    @final
    class String:
        """String value"""

        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> str: ...
        def __getitem__(self, key: int, /) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: str) -> XmpValue.String: ...
        def as_str(self, /) -> str:
            """Get the value as a string."""
        def as_int(self, /) -> int | None:
            """Get the value as an integer, if it can be converted to."""
        def as_bool(self, /) -> bool | None:
            """Get the value as a boolean, if it can be converted to."""

    @final
    class Structure:
        """Structure (key-value pairs)"""

        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> dict[str, XmpValueType]: ...
        def __getitem__(self, key: int, /) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: dict[str, XmpValueType]) -> XmpValue.Structure: ...
        def as_structure(self, /) -> dict[str, XmpValueType]:
            """Get the value as a dict."""

@final
class NamespaceMap:
    def __new__(cls, /) -> NamespaceMap:
        """Create a new namespace map with built-in namespaces registered"""
    def get_all_namespaces(self, /) -> list[tuple[str, str]]:
        """Get all registered namespaces as a list of (uri, prefix) tuples"""
    def get_prefix(self, /, uri: str) -> str | None:
        """Get the prefix for a namespace `uri`"""
    def get_uri(self, /, prefix: str) -> str | None:
        """Get the URI for a namespace `prefix`"""
    def has_prefix(self, /, prefix: str) -> bool:
        """Check if a namespace `prefix` is registered"""
    def has_uri(self, /, uri: str) -> bool:
        """Check if a namespace `uri` is registered"""
    def register(self, /, uri: str, prefix: str) -> None:
        """Register a namespace URI with a prefix

        Args:
            uri: The namespace URI
            prefix: The namespace prefix

        Raises:
            XmpError: If the prefix is already registered to a different URI
        """

def get_all_registered_namespaces() -> list[tuple[str, str]]:
    """Get all registered namespaces from global registry

    Returns a list of (uri, prefix) tuples for all registered namespaces.
    """

def get_builtin_namespace_uris() -> list[str]:
    """Get all built-in namespace URIs

    Returns a list of built-in namespace URIs.
    """

def get_global_namespace_prefix(uri: str) -> str | None:
    """Get the prefix for a namespace `uri` from global registry"""

def get_global_namespace_uri(prefix: str) -> str | None:
    """Get the URI for a namespace `prefix` from global registry"""

def is_namespace_registered(uri: str) -> bool:
    """Check if a namespace `uri` is registered globally"""

def register_namespace(uri: str, prefix: str) -> None:
    """Register a namespace URI with a prefix

    This is a convenience function that uses a global namespace map.
    For per-instance namespace management, use [`NamespaceMap`][] directly.

    This function registers namespaces globally (per thread) for convenience.
    """

def __getattr__(name: str) -> Incomplete: ...
