fn main() {
    println!("cargo:rustc-link-search=./library/");
    println!("cargo:rustc-link-lib=static=stm32f3xx_hal_gpio");
}
