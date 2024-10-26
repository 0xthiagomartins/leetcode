class TwoSum:
    def brute_force(self, nums: list[int], target: int) -> list[int]:
        for i1, num1 in enumerate(nums):
            for i2, num2 in enumerate(nums):
                if num1 + num2 == target:
                    if i1 != i2:
                        return [i1, i2]

    def two_pass_hash_table(self, nums: list[int], target: int) -> list[int]:
        lookup = {}
        for i in range(len(nums)):
            lookup[nums[i]] = i
        for i in range(len(nums)):
            diff = target - nums[i]
            if diff in lookup and lookup[diff] != i:
                return [i, lookup[diff]]

    def one_pass_hash_table(self, nums: list[int], target: int) -> list[int]:
        lookup = {}
        for i, num in enumerate(nums):
            diff = target - num
            if diff in lookup:
                return [lookup[diff], i]
            lookup[num] = i


if __name__ == "__main__":
    solution = TwoSum()

    for approach in [
        solution.brute_force,
        solution.two_pass_hash_table,
        solution.one_pass_hash_table,
    ]:
        print(f"=== Testing approach {approach.__name__} ===")
        # Test case 1
        nums1 = [2, 7, 11, 15]
        target1 = 9
        assert approach(nums1, target1) == [0, 1]
        print(f"Test case 1 passed")

        # Test case 2
        nums2 = [3, 2, 4]
        target2 = 6
        assert approach(nums2, target2) == [1, 2]
        print(f"Test case 2 passed")

        # Test case 3
        nums3 = [3, 3]
        target3 = 6
        assert approach(nums3, target3) == [0, 1]
        print(f"Test case 3 passed\n\n")
