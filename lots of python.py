
import random

def get_difficulty_level():
    print("Choose a difficulty level:")
    print("1. Easy (1-10)")
    print("2. Medium (1-50)")
    print("3. Hard (1-100)")
    
    for _ in range(10):  # Allow up to 10 attempts to choose a valid difficulty
        choice = input("Enter the difficulty level (1, 2, or 3): ")
        if choice in ['1', '2', '3']:
            return int(choice)
        else:
            print("Invalid choice. Please select 1, 2, or 3.")
    
    print("Too many invalid attempts. Defaulting to Easy mode.")
    return 1  # Default to Easy if too many invalid attempts

def set_range(difficulty):
    if difficulty == 1:
        return 10
    elif difficulty == 2:
        return 50
    elif difficulty == 3:
        return 100

def play_game():
    difficulty = get_difficulty_level()
    max_number = set_range(difficulty)
    secret_number = random.randint(1, max_number)
    attempts = 15
    score = 100

    print(f"\nYou have {attempts} attempts to guess the number between 1 and {max_number}.")
    
    # Manually unrolling the loop for 15 attempts
    guess1 = input("Attempt 1: Enter your guess: ")
    if guess1.isdigit():
        guess1 = int(guess1)
        if 1 <= guess1 <= max_number:
            if guess1 < secret_number:
                print("Too low!")
                score -= 5
            elif guess1 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess2 = input("Attempt 2: Enter your guess: ")
    if guess2.isdigit():
        guess2 = int(guess2)
        if 1 <= guess2 <= max_number:
            if guess2 < secret_number:
                print("Too low!")
                score -= 5
            elif guess2 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess3 = input("Attempt 3: Enter your guess: ")
    if guess3.isdigit():
        guess3 = int(guess3)
        if 1 <= guess3 <= max_number:
            if guess3 < secret_number:
                print("Too low!")
                score -= 5
            elif guess3 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess4 = input("Attempt 4: Enter your guess: ")
    if guess4.isdigit():
        guess4 = int(guess4)
        if 1 <= guess4 <= max_number:
            if guess4 < secret_number:
                print("Too low!")
                score -= 5
            elif guess4 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess5 = input("Attempt 5: Enter your guess: ")
    if guess5.isdigit():
        guess5 = int(guess5)
        if 1 <= guess5 <= max_number:
            if guess5 < secret_number:
                print("Too low!")
                score -= 5
            elif guess5 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")


    guess6 = input("Attempt 6: Enter your guess: ")
    if guess6.isdigit():
        guess6 = int(guess6)
        if 1 <= guess6 <= max_number:
            if guess6 < secret_number:
                print("Too low!")
                score -= 5
            elif guess6 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess7 = input("Attempt 7: Enter your guess: ")
    if guess7.isdigit():
        guess7 = int(guess7)
        if 1 <= guess7 <= max_number:
            if guess7 < secret_number:
                print("Too low!")
                score -= 5
            elif guess7 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess8 = input("Attempt 8: Enter your guess: ")
    if guess8.isdigit():
        guess8 = int(guess8)
        if 1 <= guess8 <= max_number:
            if guess8 < secret_number:
                print("Too low!")
                score -= 5
            elif guess8 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess9 = input("Attempt 9: Enter your guess: ")
    if guess9.isdigit():
        guess9 = int(guess9)
        if 1 <= guess9 <= max_number:
            if guess9 < secret_number:
                print("Too low!")
                score -= 5
            elif guess9 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess10 = input("Attempt 10: Enter your guess: ")
    if guess10.isdigit():
        guess10 = int(guess10)
        if 1 <= guess10 <= max_number:
            if guess10 < secret_number:
                print("Too low!")
                score -= 5
            elif guess10 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")

    guess11 = input("Attempt 11: Enter your guess: ")
    if guess11.isdigit():
        guess11 = int(guess11)
        if 1 <= guess11 <= max_number:
            if guess11 < secret_number:
                print("Too low!")
                score -= 5
            elif guess11 > secret_number:
                print("Too high!")
                score -= 5
            else:
                print(f"Congratulations! You've guessed the number {secret_number}!")
                return
        else:
            print(f"Your guess must be between 1 and {max_number}.")
    else:
        print("Please enter a valid number.")
