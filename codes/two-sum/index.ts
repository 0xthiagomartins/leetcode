export class Solution {
    twoSum(nums: number[], target: number): number[] | undefined {
        const lookup: { [key: number]: number } = {};
        for (let i = 0; i < nums.length; i++) {
            const num: number = nums[i];
            const diff: number = target - num;
            if (diff in lookup) {
                return [lookup[diff], i];
            }
            lookup[num] = i;
        }
        return undefined;
    }
}

(() => {
    const solution = new Solution();

    // Test case 1
    const nums1: number[] = [2, 7, 11, 15];
    const target1: number = 9;
    console.assert(JSON.stringify(solution.twoSum(nums1, target1)) === JSON.stringify([0, 1]), 'Test case 1 failed');

    // Test case 2
    const nums2: number[] = [3, 2, 4];
    const target2: number = 6;
    console.assert(JSON.stringify(solution.twoSum(nums2, target2)) === JSON.stringify([1, 2]), 'Test case 2 failed');

    // Test case 3
    const nums3: number[] = [3, 3];
    const target3: number = 6;
    console.assert(JSON.stringify(solution.twoSum(nums3, target3)) === JSON.stringify([0, 1]), 'Test case 3 failed');
})();
