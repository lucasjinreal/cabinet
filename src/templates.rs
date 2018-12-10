// all templates goes here

pub fn get_template_by_name(name: &str) -> String{

    // define some variables contains template
    let a = "CC = g++
CFLAGS = -g -Wall
SRCS = example.cc
PROG = example

OPENCV = `pkg-config opencv --cflags --libs`
LIBS = $(OPENCV)

$(PROG):$(SRCS)
	$(CC) $(CFLAGS) -o $(PROG) $(SRCS) $(LIBS)";


    let b = r#"cmake_minimum_required(VERSION 3.8)

project(a)


file(GLOB_RECURSE sources
    "src/*.cpp"
    "src/*.cc"
    "include/*.hpp"
)

find_package(OpenCV)
if(NOT OpenCV_FOUND)
    message(FATAL_ERROR "not find opencv.")
endif()

add_executable(main main.cpp ${sources})
target_link_libraries(main ${sources} ${OpenCV_LBS})"#;


    println!("Obtain template for: {}\n", name);
    if name == "makefile" {
        return a.to_string();
    } else if name == "cmake" {
        return b.to_string();
    } else {
        return format!("this template: {} is not support for now.", name);
    }
}