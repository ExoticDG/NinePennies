def first_queries():

    print("Hello and welcome to NinePennies! I am your helpful CLI array compute agent!")

    user_name = input("Please enter your name: ").strip()
    print(f"Hello, {user_name}! Nine in binary is 1001, My real name is 1001Pennies!")

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
    Performs the character replacement and counts the occurrences.
    """
    # Using a list of integers (0, 1, 2, 3) to represent the characters
    # for memory efficiency instead of storing strings.
    # 0: first_character, 1: second_character, etc.
    result_sequence = [0] * total_length

    for i in range(1, total_length, 2): # Every 2nd element (index 1, 3, 5...)
        result_sequence[i] = 1
    
    for i in range(2, total_length, 3): # Every 3rd element (index 2, 5, 8...)
        result_sequence[i] = 2

    for i in range(3, total_length, 4): # Every 4th element (index 3, 7, 11...)
        result_sequence[i] = 3

    # Count occurrences of each number
    first_count = result_sequence.count(0)
    second_count = result_sequence.count(1)
    third_count = result_sequence.count(2)
    fourth_count = result_sequence.count(3)

    result_string = (
        f"{first_char}: {first_count}, {second_char}: {second_count}, "
        f"{third_char}: {third_count}, {fourth_char}: {fourth_count}"
    )

    return result_string, first_count, second_count, third_count, fourth_count

def response_first(first_result, user_name, char_map):
    """
    Displays the first result and asks the user if they want to continue.
    """
    print(f"\nAlright {user_name}, here's how many times each character appears in the array: {first_result}")

    while True:
        continue_to_total = input("\nWould you like to continue calculations? This would solve for a total based on inputs you give for each character. (y/n)").lower().strip()
        if continue_to_total in ['y', 'n']:
            break
        print("Invalid input. Please enter 'y' or 'n'.")

    if continue_to_total == 'n':
        print("Goodbye!")
        sys.exit(0)

    print("\nAlright! Now I need a numerical value for each character.")
    
    values = {}
    for i, char_name in enumerate(char_map):
        while True:
            try:
                val_str = input(f"Please enter the value for '{char_map[char_name]}': ")
                values[char_name] = int(val_str)
                break
            except ValueError:
                print("Invalid input. Please enter a valid integer.")
    
    return values['first'], values['second'], values['third'], values['fourth']

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

def response_second(total, first_total, second_total, third_total, fourth_total, user_name, char_map):
    """
    Displays the final calculated totals.
    """
    print(f"\n{user_name}, here are the final results:")
    print(f"Total value from '{char_map['first']}': {first_total}")
    print(f"Total value from '{char_map['second']}': {second_total}")
    print(f"Total value from '{char_map['third']}': {third_total}")
    print(f"Total value from '{char_map['fourth']}': {fourth_total}")
    print(f"-----------------------------------")
    print(f"The final combined total is: {total}")


def main():
    """
    Main function to run the program.
    """
    total_length, first_char, second_char, third_char, fourth_char, user_name = first_queries()

    char_map = {
        'first': first_char,
        'second': second_char,
        'third': third_char,
        'fourth': fourth_char
    }

    first_result, first_count, second_count, third_count, fourth_count = calculate_first_query(
        total_length, first_char, second_char, third_char, fourth_char
    )

    first_val, second_val, third_val, fourth_val = response_first(first_result, user_name, char_map)

    total, first_total, second_total, third_total, fourth_total = calculate_second_query(
        first_val, second_val, third_val, fourth_val,
        first_count, second_count, third_count, fourth_count
    )

    response_second(total, first_total, second_total, third_total, fourth_total, user_name, char_map)


if __name__ == "__main__":
    main()
