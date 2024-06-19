# journald-export-parser-rs

* [crates.io](https://crates.io/crates/journald-export-parser-rs)
* [docs.rs](https://docs.rs/journald-export-parser-rs/latest/journald_export_parser_rs/)

```
journald-export-parser-rs = "0.1.0"
```

# Summary

This is a small, _strict_ parser for the systemd [journal export
format](https://systemd.io/JOURNAL_EXPORT_FORMATS/). The parser avoids heap
allocations and operates on a bounded buffer. The length of each journal entry
and each field is bounded and can be configured.

This library should be useful in situations where many different streams are
consumed simultaneously—and, thus, memory efficiency is a priority—and/or where
the parsed journal entries are further reduced right away. For example, when
analyzing logs, often only a few fields are of interest while the rest is
ignored. Another example: numeric fields are parsed into more compact
representations (such as integer values).

See the
[documentation](https://docs.rs/journald-export-parser-rs/latest/journald_export_parser_rs/)
for more information.