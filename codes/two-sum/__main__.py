class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        lookup = {}
        for i, num in enumerate(nums):
            diff = target - num
            if diff in lookup:
                return [lookup[diff], i]
            lookup[num] = i


if __name__ == "__main__":
    solution = Solution()

    # Test case 1
    nums1 = [2, 7, 11, 15]
    target1 = 9
    assert solution.twoSum(nums1, target1) == [0, 1]

    # Test case 2
    nums2 = [3, 2, 4]
    target2 = 6
    assert solution.twoSum(nums2, target2) == [1, 2]

    # Test case 3
    nums3 = [3, 3]
    target3 = 6
    assert solution.twoSum(nums3, target3) == [0, 1]
