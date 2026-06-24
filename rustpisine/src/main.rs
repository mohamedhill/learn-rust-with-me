
    fn main() {
        //shdowing example 
        let age = 10;
        
        let  age = age *10;
        println!("my age {}",age);



        // in const you sould put the type to declare an constant

        const test: i32 = 10;
        let  test2 =10_u8;
        println!("{}",test);


       // rust Data types


       // integer , float , bool , char
       // rust can define the data type based on the value but not always 


       // here thz type of ints 
        // int
       // i8 ,i16 , i32 , i64 , i128 positive and negative
       // u8 ,u16 , u32 , u64 , u128 just a positive int
       //isize , usize
        //flout

        //f32 ,f64

         // char = 4 byte

         //tuple,arrays

        //tuple
         let data = (32,'A',true,"hello");
        //array
        // array stock in heap and the size is static 100%
        let array = [1,10,11,2];
         

         let tesarray =[10;10];
         //output
       //  [10,10,10,10,10,10,10,10,10,10] 



        println!("here is the sume {}",testfunc(10,30));

        //println!("{}",unitfunc());
        let test:i32 = 10;

        loops();
    }


    fn testfunc(mut a:i32,mut b:i32)->i32 {
        let mut sum : i32 = 0;
        while b>0 {
            sum+=a;
            b-=1;

            
        }


        for i in 'a'..'z'{
            println!("here tha char {}",i)

        }
        return sum;
    }


    fn unitfunc(){
        let test: i32 = 10 ;


        
    }

    fn loops(){

      //loops in rust 
        //while , for , loop
let mut count = 0 ;
    let res: i32 = loop {
     println!("infinte loop");
     count+=1;
    if count == 5{
        break 10;
    }
      };



    }
