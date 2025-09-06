import sys


import sys


def first_queries():

    print("Hello and welcome to NinePennies! I am your helpful CLI array compute agent!")

    user_name = input("Please enter your name: ").strip()
    print("Hello, {}! Nine in binary is 1001, My real name is 1001Pennies!".format(user_name))

    print("\nI replace characters in an array and give you the total of each occurrence.")
    print("First I replace every second character, then every third character, and finally every fourth character.")
    print("Once I have the result, I tell you how many times each replaced character occurred in the array I made.")
    print("Let's get started!\n")

    while True:
        try:
            total_length_str = input("First, I need the total length of your sequence: ")
            total_length = int(total_length_str)
            if total_length > 0:
                break
            else:
                print("Please enter a positive number.")
        except ValueError:
            print("Invalid input. Please enter a valid integer.")

    first_character = input("Next, I need the first character you want me to use. This will be the base character that then gets replaced by the other ones: ").strip()
    second_character = input("Now, I need the second character. This replaces every second character in the array: ").strip()
    third_character = input("After that I of course need the third character. You get how this works by now, right? This replaces every third character in the array: ").strip()
    fourth_character = input("Finally, I need the fourth character: ").strip()

    return total_length, first_character, second_character, third_character, fourth_character, user_name

def calculate_first_query(total_length, first_char, second_char, third_char, fourth_char):
    """
    Calculates the occurrences of each character without storing the full sequence in memory.
    This is more memory-efficient for devices with limited RAM.
    """
    first_count = 0
    second_count = 0
    third_count = 0
    fourth_count = 0

    for i in range(1, total_length + 1):
        if i % 4 == 0:
            fourth_count += 1
        elif i % 3 == 0:
            third_count += 1
        elif i % 2 == 0:
            second_count += 1
        else:
            first_count += 1
    
    result_string = "{}: {}, {}: {}, {}: {}, {}: {}".format(
        first_char, first_count, second_char, second_count,
        third_char, third_count, fourth_char, fourth_count
    )

    return result_string, first_count, second_count, third_count, fourth_count

def response_first(first_result, user_name, first_char, second_char, third_char, fourth_char):
    """
    Displays the first result and asks the user if they want to continue.
    """
    print("\nAlright {}, here's how many times each character appears in the array: {}".format(user_name, first_result))

    while True:
        continue_to_total = input("\nWould you like to continue calculations? (y/n)").lower().strip()
        if continue_to_total in ['y', 'n']:
            break
        print("Invalid input. Please enter 'y' or 'n'.")

    if continue_to_total == 'n':
        print("Goodbye!")
        return None # Return None to signal exiting

    print("\nAlright! Now I need a numerical value for each character.")
    
    while True:
        try:
            val1_str = input("Please enter the value for '{}': ".format(first_char))
            first_val = int(val1_str)
            break
        except ValueError:
            print("Invalid input. Please enter a valid integer.")
            
    while True:
        try:
            val2_str = input("Please enter the value for '{}': ".format(second_char))
            second_val = int(val2_str)
            break
        except ValueError:
            print("Invalid input. Please enter a valid integer.")

    while True:
        try:
            val3_str = input("Please enter the value for '{}': ".format(third_char))
            third_val = int(val3_str)
            break
        except ValueError:
            print("Invalid input. Please enter a valid integer.")

    while True:
        try:
            val4_str = input("Please enter the value for '{}': ".format(fourth_char))
            fourth_val = int(val4_str)
            break
        except ValueError:
            print("Invalid input. Please enter a valid integer.")
    
    return first_val, second_val, third_val, fourth_val

def calculate_second_query(first_value, second_value, third_value, fourth_value, first_count, second_count, third_count, fourth_count):
    """
    Calculates the total value based on character counts and their assigned values.
    """
    first_total = first_value * first_count
    second_total = second_value * second_count
    third_total = third_value * third_count
    fourth_total = fourth_value * fourth_count

    total = first_total + second_total + third_total + fourth_total

    return total, first_total, second_total, third_total, fourth_total

def response_second(total, first_total, second_total, third_total, fourth_total, user_name, first_char, second_char, third_char, fourth_char):
    """
    Displays the final calculated totals.
    """
    print("\n{}, here are the final results:".format(user_name))
    print("Total value from '{}': {}".format(first_char, first_total))
    print("Total value from '{}': {}".format(second_char, second_total))
    print("Total value from '{}': {}".format(third_char, third_total))
    print("Total value from '{}': {}".format(fourth_char, fourth_total))
    print("-----------------------------------")
    print("The final combined total is: {}".format(total))


def main():
    """
    Main function to run the program.
    """
    total_length, first_char, second_char, third_char, fourth_char, user_name = first_queries()

    first_result, first_count, second_count, third_count, fourth_count = calculate_first_query(
        total_length, first_char, second_char, third_char, fourth_char
    )

    # The response_first function now returns a tuple of values, or None if the user quits.
    values = response_first(first_result, user_name, first_char, second_char, third_char, fourth_char)
    
    # If user chose not to continue, exit the main function.
    if values is None:
        return
        
    first_val, second_val, third_val, fourth_val = values

    total, first_total, second_total, third_total, fourth_total = calculate_second_query(
        first_val, second_val, third_val, fourth_val,
        first_count, second_count, third_count, fourth_count
    )

    response_second(total, first_total, second_total, third_total, fourth_total, user_name, first_char, second_char, third_char, fourth_char)

# Run the main function
main()
