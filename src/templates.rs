// all templates goes here

pub fn get_template_by_name(name: &str) -> String{

    // define some variables contains template
    let a = "CC = g++
CFLAGS = -g -Wall
SRCS = r.cpp
PROG = example

# based on usage, libaries can be add or remove
OPENCV = `pkg-config opencv --cflags --libs`
THOR = -lthor
PCL = -lpcl_io -lpcl_visualization -lpcl_common -lpcl_features
# this should got from `locate vtkCommonCore` with various version
VTK = -I/usr/local/include/vtk -lvtkCommonCore-8.1
BOOST = -lboost_regex -lboost_system
LIBS = $(OPENCV) $(THOR) $(PCL) $(VTK) $(BOOST)

$(PROG):$(SRCS)
	$(CC) $(CFLAGS) -o $(PROG) $(SRCS) $(LIBS)
";


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
