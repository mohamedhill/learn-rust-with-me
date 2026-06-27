# learn-rust-with-me
learn rust with me

# start with w3shcools 
https://www.w3schools.com/rust/rust_variables.php


i learn from the playlist var and const and data types

in rust we use let to declare variable and in the defualt it will be imutable 
and you cant assign it the error : 
# cannot assign twice to immutable variable
const you sould declare it with the type 
or you will see the typerror

# i sould learn more about unit type and usize and isize


let x = {
    let a = 5;
};

x = ()
 here if we write like this with the symcolon the block vlue should be the unit type

let x = {
    let a = 5;
    a
};

x = 5 

every thing return a exprsion even the void func it return the unit type


; = in rust do this thing and ignore the value 


example :

let x = {
    5
};
output 
x = 5

let x = {
    5;
};
output 
x = ()



#loops

while , for , loop


today i learn as is to covert type from i32 to f64 par example




2574. Left and Right Sum Differences
Solved
Easy
Topics
premium lock icon
Companies
Hint
You are given a 0-indexed integer array nums of size n.

Define two arrays leftSum and rightSum where:

leftSum[i] is the sum of elements to the left of the index i in the array nums. If there is no such element, leftSum[i] = 0.
rightSum[i] is the sum of elements to the right of the index i in the array nums. If there is no such element, rightSum[i] = 0.
Return an integer array answer of size n where answer[i] = |leftSum[i] - rightSum[i]|.

 

Example 1:

Input: nums = [10,4,8,3]
Output: [15,1,11,22]
Explanation: The array leftSum is [0,10,14,22] and the array rightSum is [15,11,3,0].
The array answer is [|0 - 15|,|10 - 11|,|14 - 3|,|22 - 0|] = [15,1,11,22].
Example 2:

Input: nums = [1]
Output: [0]
Explanation: The array leftSum is [0] and the array rightSum is [0].
The array answer is [|0 - 0|] = [0].
 //////////////////////////////////////:
 # string methods 


 push : add one char to last 

 push_str : push sentence 

 let mut s = String::from("Hello");

s.push_str(" World");

println!("{}", s);



# empty ?
let s = String::new();

println!("{}", s.is_empty());

remove the last char 

let mut s = String::from("Hello");

s.pop();

println!("{}", s); // Hell


remove with index 


let mut s = String::from("Hello");

s.remove(1);

println!("{}", s); // Hllo


insert with index 

let mut s = String::from("Hllo");

s.insert(1, 'e');

println!("{}", s);


add string 

let mut s = String::from("Hello");

s.insert_str(5, " World");

println!("{}", s);


remove str 

let mut s = String::from("Hello");

s.clear();

println!("{}", s); // ""

replace 

let s = "Hello World";

let new = s.replace("World", "Rust");

println!("{}", new);