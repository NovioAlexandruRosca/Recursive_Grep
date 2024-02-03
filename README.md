# Project Title: Recursive_Grep made in Rust

## Creation Interval.
December 10, 2023 - December 17, 2023

## Description
This project, Recursive_Grep, is a command-line utility implemented in Rust. The utility is designed to recursively search through directories and files, providing users with powerful pattern matching capabilities for valid UTF-8 written files.
With Recursive_Grep, users can efficiently search for specific strings or patterns within their file system, making it a valuable tool for developers, sysadmins, and anyone working with text-based data.
Utilizing Rust's performance and safety features, Recursive_Grep offers fast and reliable searching while ensuring code integrity and security.
Whether used for codebase exploration, log analysis, or content management, Recursive_Grep provides a versatile and efficient solution for searching and analyzing text data.

## How to Use

Example: `cargo run <string_to_search> -i -c -r -n 10 <folder_to_be_searched>`

### Command Line Arguments:

- `<string_to_search>` (Mandatory): The string that's supposed to be searched for.
- `-n <number>`: Specifies the maximum number of lines to search through. Default is infinite.
- `-i`: Ignores case when searching. Default is off.
- `-c`: Enables "count-only" mode, which prints only the number of matches in a file. Default is off.
- `-r`: Enables regex searching for advanced pattern matching. Default is off.
- `<folder_to_be_searched>`(Mandatory): The name of the folder to be searched for matching strings.

You can find other examples after Dependencies

## Features

- **Custom Search String**: Users can specify a custom string to search for within the schemas.
- **Maximum Lines Limit**: Users can set a maximum limit for the number of lines to search through. The application exits after reaching this limit. The default is infinite.
- **Case Insensitivity**: Users have the option to perform case-insensitive searches. The default is case-sensitive.
- **Count-Only Mode**: Users can enable a mode where only the count of matches per file is printed, without displaying the matching lines. The default is off.
- **Regular Expression Support**: Option to enable regular expression searching for advanced pattern matching.
- **Enhanced User Experience**: The use of the colored library enhances the user experience by displaying output text in different colors and formats.
- **+help Subcommand**: Users can use the +help command to access instructions on how to use the application.
- **Comprehensive Error Handling**: Robust error handling implemented to cover all scenarios.(Implemented with thisError)

## Dependencies

- **regex** (Version 1.10.2)
- **thiserror** (Version 1.0)
- **colored** (Version 2.0.0)

Here you can see a couple of screenshots of the app:

- **Command cargo run +help**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/6ebf7d92-4555-4d66-bb6c-1f4536bef216)

- **App in-use**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/779850c0-d0ef-4b03-a8c2-aa816d21b26a)
- **Attribute -n 2 (Maximum 2 lines are checked before the app is terminated)**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/6e5439a7-ccc6-40c2-a7d3-5c8a212cac90)
- **Attribute -i (Case insensitive search)**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/5f538bdd-552d-4d9d-b90b-6a25a64142b9)
- **Attribute -c(Only count how many times the searched string appears))**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/d6a08f0b-225f-4fcd-83fe-7c4a01aa41db)
- **Attribute -r(Allow regex searching)**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/0976f439-2bba-45b9-b8f3-4be672588366)
- **Possible Absence of Searched String in Folder (Notification)**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/a4bdfe64-587c-4a60-ae8b-43ad327a140e)
- **Possible Errors (Non-Exhaustive List)**
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/e2fd2d9b-f1e1-44ba-95d2-09f2d1043132)
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/388a6987-fd91-47ab-8614-b2b87a804c40)
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/f5bcb1aa-5d2b-4205-b469-02e5257135f6)
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/a4ca2658-de51-4ecd-a9ff-0fc2bfc50973)
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/6586a706-6225-4ac6-b4cb-74a19771c974)
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/a8813cec-ca4b-4687-bb7a-497ece6d6bcb)
![image](https://github.com/AlexandruRoscaPOO/Recursive_Grep/assets/113398639/9debd02d-2f58-4aef-bdba-af6a1d8353f7)




