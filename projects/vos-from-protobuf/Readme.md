

```protobuf
message FooMessage {
    required int32 b_required_int32 = 1;
    optional int32 f_int32 = 1;
    repeated int32 f_repeated_int32 = 19;
    required FooMessage f_self_message = 17;
}
```


```vos
table FooMessage {
    b_required_int32: i32;
    f_int32: f32?;
    f_repeated_int32: Vector[i32]
    f_self_message: Self
}
```


```protobuf
enum FooEnum {
    FIRST_VALUE = 1;
    SECOND_VALUE = 2;
}
```


```vos
union FooEnum {
    FIRST_VALUE = 1;
    SECOND_VALUE = 2;
}
```