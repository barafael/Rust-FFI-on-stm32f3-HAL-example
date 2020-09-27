# C FFI on an stm32f3 discovery (Cortex-M4) using the STM32 HAL

In library/, there are the Driver files for stm32f3. They are only needed to generate the bindings.rs file.

For now, generate the bindings by changing to library/ directory and running: 

<code>
bindgen Drivers/STM32F3xx_HAL_Driver/Inc/stm32f3xx_hal_gpio.h --use-core --ctypes-prefix="libc" --raw-line="#[no_std]" --raw-line="mod libc { pub type c_uint = u32;\
          pub type c_int = i32;\
          pub type c_char = i8;\
          pub type c_uchar = u8;\
          pub type c_short = i16;\
          pub type c_ushort = u16;\
          pub type c_long = i64;\
          pub type c_ulong = u64;\
          pub type c_schar = i8;\
          pub type c_longlong = i64;\
          pub type c_ulonglong = u64;pub enum c_void {}}" -- -I Drivers/CMSIS/Include/ -I Drivers/CMSIS/Device/ST/STM32F3xx/Include/ -I Drivers/STM32F3xx_HAL_Driver/Inc/ -I Drivers/CMSIS/Device/ST/STM32F3xx/Include/stm32f3xx.h -DSTM32F3 -DSTM32F303xC > ../src/bindings.rs
</code>
