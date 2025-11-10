//
// Created by aemilia on 11/9/25.
//

#include <iostream>
#include <vector>
#include <cmath>

using namespace std;

// Class: data + functions that operate on that data
class Vector3D {
private:
    // Member variables (data)
    double x, y, z;

public:
    // Constructor - called when object is created
    Vector3D(double x_val, double y_val, double z_val)
        : x(x_val), y(y_val), z(z_val) {}

    // Member functions (methods)
    double magnitude() const {
        return sqrt(x*x + y*y + z*z);
    }

    Vector3D add(const Vector3D& other) const {
        return Vector3D(x + other.x, y + other.y, z + other.z);
    }

    void scale(double factor) {
        x *= factor;
        y *= factor;
        z *= factor;
    }

    void print() const {
        cout << "(" << x << ", " << y << ", " << z << ")";
    }

    // Getters
    double get_x() const { return x; }
    double get_y() const { return y; }
    double get_z() const { return z; }
};

// Another example: Statistics tracker
class Stats {
private:
    vector<double> data;

public:
    // Add data point
    void add(double value) {
        data.push_back(value);
    }

    // Compute mean
    double mean() const {
        if (data.empty()) return 0.0;

        double sum = 0.0;
        for (double val : data) {
            sum += val;
        }
        return sum / data.size();
    }

    // Find min
    double min() const {
        if (data.empty()) return 0.0;

        double min_val = data[0];
        for (double val : data) {
            if (val < min_val) min_val = val;
        }
        return min_val;
    }

    // Find max
    double max() const {
        if (data.empty()) return 0.0;

        double max_val = data[0];
        for (double val : data) {
            if (val > max_val) max_val = val;
        }
        return max_val;
    }

    // Get count
    size_t count() const {
        return data.size();
    }

    // Print summary
    void print_summary() const {
        cout << "Count: " << count() << endl;
        cout << "Mean:  " << mean() << endl;
        cout << "Min:   " << min() << endl;
        cout << "Max:   " << max() << endl;
    }
};

int main() {
    cout << "=== Vector3D Class ===" << endl;

    Vector3D v1(1.0, 2.0, 3.0);
    Vector3D v2(4.0, 5.0, 6.0);

    cout << "v1 = ";
    v1.print();
    cout << ", magnitude = " << v1.magnitude() << endl;

    cout << "v2 = ";
    v2.print();
    cout << ", magnitude = " << v2.magnitude() << endl;

    Vector3D v3 = v1.add(v2);
    cout << "v1 + v2 = ";
    v3.print();
    cout << endl;

    v1.scale(2.0);
    cout << "v1 scaled by 2 = ";
    v1.print();
    cout << endl;

    cout << "\n=== Stats Class ===" << endl;

    Stats sensor_data;

    // Add measurements
    sensor_data.add(23.5);
    sensor_data.add(24.1);
    sensor_data.add(22.8);
    sensor_data.add(25.3);
    sensor_data.add(23.9);

    sensor_data.print_summary();

    cout << "\n=== Why Classes Matter for HPC ===" << endl;
    cout << "1. Organize complex simulations (particles, grids, etc)" << endl;
    cout << "2. No performance penalty - methods inline like C functions" << endl;
    cout << "3. Easier to maintain large codebases" << endl;
    cout << "4. Can optimize entire class at once" << endl;

    return 0;
}