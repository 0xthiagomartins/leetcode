import java.util.HashMap;
import java.util.Arrays;

public class Main {
    public int[] bruteForce(int[] nums, int target) {
        for (int i = 0; i < nums.length; i++) {
            for (int j = 0; j < nums.length; j++) {
                if (nums[i] + nums[j] == target && i != j) {
                    return new int[]{i, j};
                }
            }
        }
        return new int[]{};
    }

    public int[] twoPassHashTable(int[] nums, int target) {
        HashMap<Integer, Integer> lookup = new HashMap<>();
        
        // First pass: build the hash table
        for (int i = 0; i < nums.length; i++) {
            lookup.put(nums[i], i);
        }
        
        // Second pass: find the complement
        for (int i = 0; i < nums.length; i++) {
            int diff = target - nums[i];
            if (lookup.containsKey(diff) && lookup.get(diff) != i) {
                return new int[]{i, lookup.get(diff)};
            }
        }
        return new int[]{};
    }

    public int[] onePassHashTable(int[] nums, int target) {
        HashMap<Integer, Integer> lookup = new HashMap<>();
        
        for (int i = 0; i < nums.length; i++) {
            int diff = target - nums[i];
            if (lookup.containsKey(diff)) {
                return new int[]{lookup.get(diff), i};
            }
            lookup.put(nums[i], i);
        }
        return new int[]{};
    }

    public static void main(String[] args) {
        Main solution = new Main();

        // Test cases
        int[][] testNums = {
            {2, 7, 11, 15},
            {3, 2, 4},
            {3, 3}
        };
        int[] testTargets = {9, 6, 6};
        int[][] expectedResults = {
            {0, 1},
            {1, 2},
            {0, 1}
        };

        // Test each approach
        for (String approach : Arrays.asList("bruteForce", "twoPassHashTable", "onePassHashTable")) {
            System.out.println("=== Testing approach " + approach + " ===");
            
            for (int i = 0; i < testNums.length; i++) {
                int[] result;
                switch (approach) {
                    case "bruteForce":
                        result = solution.bruteForce(testNums[i], testTargets[i]);
                        break;
                    case "twoPassHashTable":
                        result = solution.twoPassHashTable(testNums[i], testTargets[i]);
                        break;
                    case "onePassHashTable":
                        result = solution.onePassHashTable(testNums[i], testTargets[i]);
                        break;
                    default:
                        result = new int[]{};
                }
                
                assert Arrays.equals(result, expectedResults[i]) : 
                    "Test case " + (i + 1) + " failed for " + approach;
                System.out.println("Test case " + (i + 1) + " passed");
            }
            System.out.println("\n");
        }
    }
}
