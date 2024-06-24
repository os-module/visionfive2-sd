# starfive2 SD card driver

This is a simple SD card driver for the StarFive2 board. 


## Usage
```rust
fn main(){
    fn sleep(ms:usize){}
    use visionfive2_sd::Vf2SdDriver;
    let driver = Vf2SdDriver::new(sleep);
    driver.init();
    let mut buf = [0u8;512];
    driver.read_block(0,&mut buf);
    driver.write_block(0,&buf);
}
```


