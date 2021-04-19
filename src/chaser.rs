//! A static parameter of the lights will be name below a "sequence".
//! The parade of multiple sequences will be named below as a "chaser".
//! The transition parameters between two sequences will be named below as an "effect".
//! The chaser is composed by sequences and effects.

struct Second(u32);
struct Ms(u32);

impl From<Ms> for Second {
    fn from(ms: Ms) -> Self {
        Second(ms.0 / 1000)
    }
}

impl From<Second> for Ms {
    fn from(seconds: Second) -> Self {
        Ms(seconds.0 * 1000)
    }
}

/// A struct which defines the chaser.
struct Chaser {
    count: usize,
}

impl Chaser {
    fn new() -> Chaser {
        Chaser { count: 0 }
    }
}

impl Iterator for Chaser {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        // Below : replace "6" by the calculation of the total amount of sequences.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// A sequence will be define as a variable, using a Struct as an instance.
// The type of a sequence variable will be RVB or HSV (which are Structs composed from 3 well defined unsigned integers).

// An effect will be defined as a function.
// The effect function will take a parameter of time (duration), which can itself be define in a dedicated variable.
// The other parameters of the function will be the sequences.

// The effect function calculation could looks like :

// - (sequence 1 - sequence 2)
//_____________________________
//          duration

// => That means it will calculate the change needed for the 3 RVB or HSV values
// to move from the static state defined in sequence 1 towards the static state defined in sequence 2.
// In that way, we are able to define the duration of ONE transition.
// If we want a 2min30 chaser duration, we should calculate it ourselves.

// Other effect function culculation could looks like :

//               - (sequence 1 - sequence 2)
//______________________________________________________________
// total duration of the Chaser / (total number of sequences -1)

// => That means the effect duration will be automatically calculated from the total chaser duration we chose.
// In this way, we can easily defined a total duration for our chaser,
// but we are less flexible regarding the duration of 1 effect (transition between 2 sequences).
