//! You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
//! 
//! Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//! 
//! The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
//! 
//! Example 1:
//! Input: nums1 = \[1,2,3,0,0,0\], m = 3, nums2 = \[2,5,6\], n = 3
//! Output: \[1,2,2,3,5,6\]
//! Explanation: The arrays we are merging are \[1,2,3\] and \[2,5,6\].
//! The result of the merge is \[1,2,2,3,5,6\] with the underlined elements coming from nums1.
//! 
//! Example 2:
//! Input: nums1 = \[1\], m = 1, nums2 = \[\], n = 0
//! Output: \[1\]
//! Explanation: The arrays we are merging are \[1\] and \[\].
//! The result of the merge is \[1\].
//! 
//! Example 3:
//! Input: nums1 = \[0\], m = 0, nums2 = \[1\], n = 1
//! Output: \[1\]
//! Explanation: The arrays we are merging are \[\] and \[1\].
//! The result of the merge is \[1\].
//! 
//! Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
//! 
//! Constraints:
//! nums1.length == m + n
//! nums2.length == n
//! 0 <= m, n <= 200
//! 1 <= m + n <= 200
//! -109 <= nums1\[i\], nums2\[j\] <= 109
pub struct Solution;

/// Implementation for Merge Sorted Arrays solution
impl Solution {

    /// Merges 2 sorted arrays into 1 in non-decreasing order.
    ///
    /// # Arguments
    ///
    /// * `nums1` - A mutable vector of numbers
    /// * `m` - The size of nums1
    /// * `nums2` - A mutable vector of numbers
    /// * `n` - The size of nums2
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nums1 = vec![1,2,3,0,0,0];
    /// let mut nums2 = vec![2,5,6];
    /// 
    /// // result: [1,2,3,4,5,6]
    /// merge(&mut nums1, 6, &mut nums2, 3);
    /// ```
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let mut m = m - 1;
        let mut n = n - 1;

        for i in (0..nums1.len()).rev() {
            if m < 0 || n >= 0 && nums1[m as usize] < nums2[n as usize] {
                nums1[i] = nums2[n as usize];
                n -= 1;
            } else {
                nums1[i] = nums1[m as usize];
                m -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let mut nums2 = vec![2,5,6];
        let expected = vec![1,2,2,3,5,6];

        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(expected, nums1);
    }

    #[test]
    fn example_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let expected = vec![1];

        Solution::merge(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(expected, nums1);
    }

    #[test]
    fn example_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let expected = vec![1];

        Solution::merge(&mut nums1, 1, &mut nums2, 1);

        assert_eq!(expected, nums1);
    }
}
