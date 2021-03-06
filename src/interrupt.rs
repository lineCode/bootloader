use cortex_m::peripheral::NVIC;
use stm32f4xx_hal::interrupt;
use stm32f4xx_hal::stm32;
use crate::{tim, button, led, SECOND, UPGRADE_FLAG};

/// NVIC enable
pub fn nvic_enable() {
    unsafe {
        NVIC::unmask(stm32::interrupt::TIM2);
        NVIC::unmask(stm32::interrupt::EXTI4);
    }
}

/// NVIC disable
pub fn nvic_disable() {
    NVIC::mask(stm32::interrupt::TIM2);
    NVIC::mask(stm32::interrupt::EXTI4);
}

/// handle TIM2 interrupt
#[interrupt]
fn TIM2() {
    unsafe {
        tim::clean_interrupt_flag();
        SECOND += 1;
        if SECOND == 5 { tim::disable_count(); }
    }
}


/// handle EXTI4 interrupt
#[interrupt]
fn EXTI4() {
    unsafe {
        UPGRADE_FLAG = true;
        SECOND = 5;
    }

    tim::clean_interrupt_flag();
    tim::disable_count();
    button::disable_interrupt();
    led::green_dark();
    led::red_light();
}
