//
// Created by aemilia on 11/7/25.
//

#include <iostream>
#include <vector>

using namespace std;



// Sorts integers in Array for Primes

int is_prime(int num) {
    if (num <= 1) return 0;
    if (num == 2) return 1;
    if (num % 2 == 0) return 0;


    for (int i =2; i * i <= num; i++ ) {
        if (num % i == 0) return 0;
    }
    return 1;
}



void find_prime( int* arr, int size) {
    printf("Prime Numbers:");
    for (int i = 0; i < size; i++) {
        if ( is_prime(arr[i])){
            printf("%d ", arr[i]);
        }
    }
    printf("\n");
}


//int main() {
    int size = 100;

    // Allocate Array
    std::vector<int> numbers(100);

    // Fill Array
    for (int i = 0; i < size; i++) {
        numbers[i] = i + 1;
    }


    // Find Primes
    find_prime(numbers.data(), size);


    return 0;
}