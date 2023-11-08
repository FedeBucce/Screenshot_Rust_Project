; ModuleID = 'probe4.3b282679cc950ee4-cgu.0'
source_filename = "probe4.3b282679cc950ee4-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@alloc_8b7a8e2ab76041d49c97be8ac20d133f = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fee5518cdd4435c60a57fe3bb734fc1a14abeb7a\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_339214a0af53069bf5b599140cf77478 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_8b7a8e2ab76041d49c97be8ac20d133f, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h0b8b24d288d691c2E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h51feb2660473ec25E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h0f26466564961a69E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_339214a0af53069bf5b599140cf77478) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h51feb2660473ec25E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h0f26466564961a69E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0-nightly (fee5518cd 2023-11-05)"}
