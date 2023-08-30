//! Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//! 
//! Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
//! 
//! Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
//! Return k.
//! Custom Judge:
//! 
//! The judge will test your solution with the following code:
//! 
//! int[] nums = [...]; // Input array
//! int val = ...; // Value to remove
//! int[] expectedNums = [...]; // The expected answer with correct length.
//!                             // It is sorted with no values equaling val.
//! 
//! int k = removeElement(nums, val); // Calls your implementation
//! 
//! assert k == expectedNums.length;
//! sort(nums, 0, k); // Sort the first k elements of nums
//! for (int i = 0; i < actualLength; i++) {
//!     assert nums[i] == expectedNums[i];
//! }
//! If all assertions pass, then your solution will be accepted.
//! 
//! Example 1:
//! 
//! Input: nums = [3,2,2,3], val = 3
//! Output: 2, nums = [2,2,_,_]
//! Explanation: Your function should return k = 2, with the first two elements of nums being 2.
//! It does not matter what you leave beyond the returned k (hence they are underscores).
//! Example 2:
//! 
//! Input: nums = [0,1,2,2,3,0,4,2], val = 2
//! Output: 5, nums = [0,1,4,0,3,_,_,_]
//! Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
//! Note that the five elements can be returned in any order.
//! It does not matter what you leave beyond the returned k (hence they are underscores).
//!  
//! Constraints:
//! 0 <= nums.length <= 100
//! 0 <= nums[i] <= 50
//! 0 <= val <= 100
pub struct Solution;

/// Implementation for remove element solution
impl Solution {

    /// Removes all instances of the provided number in the vector by moving the items to the end of the array
    /// and returning the length of valid characters.
    ///
    /// # Arguments
    ///
    /// * `nums` - A mutable vector of numbers
    /// * `val` - The number to remove
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nums = vec![1,3,5,3,4,6,4,5,8];
    /// let val: i32 = 3;
    /// 
    /// // result: 7
    /// // nums: [1,8,5,5,4,6,4,3,3]
    /// remove_element(&mut nums, val);
    /// ```
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 { return 0; }

        let mut i = 0 as usize;
        let mut k = nums.len() - 1 as usize;
        
        while i < k {
            if k == 0 { break; }
            if nums[k] == val {
                k -= 1;
                continue;
            }

            if nums[i] == val {
                nums.swap(i, k);
                k -= 1;
            }
            
            i += 1;
        }

        return if nums[i] == val { i as i32 } else { (i + 1) as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![3,2,2,3];
        let val = 3;
        let expected: Vec<i32> = vec![2,2];

        let result = Solution::remove_element(&mut nums, val);

    
        assert_eq!(2, result);
        assert_eq!(expected, nums[0..result as usize].to_vec());
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let expected: Vec<i32> = vec![0,1,4,0,3];

        let result = Solution::remove_element(&mut nums, val);

        assert_eq!(5, result);
        assert_eq!(expected, nums[0..result as usize].to_vec());
    }

    #[test]
    fn example_3() {
        let mut nums = vec![];
        let val = 0;
        let expected: Vec<i32> = vec![];

        let result = Solution::remove_element(&mut nums, val);

        assert_eq!(0, result);
        assert_eq!(expected, nums[0..result as usize].to_vec());
    }

    #[test]
    fn example_4() {
        let mut nums = vec![1];
        let val = 1;
        let expected: Vec<i32> = vec![];

        let result = Solution::remove_element(&mut nums, val);

        assert_eq!(0, result);
        assert_eq!(expected, nums[0..result as usize].to_vec());
    }

    #[test]
    fn example_5() {
        let mut nums = vec![3,3];
        let val = 3;
        let expected: Vec<i32> = vec![];

        let result = Solution::remove_element(&mut nums, val);

        assert_eq!(0, result);
        assert_eq!(expected, nums[0..result as usize].to_vec());
    }

    #[test]
    fn example_6() {
        let mut nums = vec![4,1,2,4,4,0,0];
        let val = 4;
        let expected: Vec<i32> = vec![0,1,2,0];

        let result = Solution::remove_element(&mut nums, val);

        assert_eq!(4, result);
        assert_eq!(expected, nums[0..result as usize].to_vec());
    }
}