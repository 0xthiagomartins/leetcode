import java.util.HashMap;
import java.util.Arrays;

public class Main {
    public int bruteForce(String s) {
        int total = 0;
        int prevValue = 0;

        for (int i = 0; i < s.length(); i++) {
            int currentValue = getValue(s.charAt(i));
            if (currentValue > prevValue) {
                total += currentValue - 2 * prevValue;
            } else {
                total += currentValue;
            }
            prevValue = currentValue;
        }
        return total;
    }

    public int hashTableApproach(String s) {
        HashMap<Character, Integer> romanValues = new HashMap<>();
        romanValues.put('I', 1);
        romanValues.put('V', 5);
        romanValues.put('X', 10);
        romanValues.put('L', 50);
        romanValues.put('C', 100);
        romanValues.put('D', 500);
        romanValues.put('M', 1000);

        int total = 0;
        int prevValue = 0;

        for (char c : s.toCharArray()) {
            int currentValue = romanValues.get(c);
            if (currentValue > prevValue) {
                total += currentValue - 2 * prevValue;
            } else {
                total += currentValue;
            }
            prevValue = currentValue;
        }
        return total;
    }

    private int getValue(char c) {
        switch (c) {
            case 'I': return 1;
            case 'V': return 5;
            case 'X': return 10;
            case 'L': return 50;
            case 'C': return 100;
            case 'D': return 500;
            case 'M': return 1000;
            default: return 0;
        }
    }

    public static void main(String[] args) {
        Main solution = new Main();

        // Test cases
        String[] testInputs = {
            "III",
            "LVIII",
            "MCMXCIV"
        };
        int[] expectedResults = {3, 58, 1994};

        // Test each approach
        for (String approach : Arrays.asList("bruteForce", "hashTableApproach")) {
            System.out.println("=== Testing approach " + approach + " ===");
            
            for (int i = 0; i < testInputs.length; i++) {
                int result;
                switch (approach) {
                    case "bruteForce":
                        result = solution.bruteForce(testInputs[i]);
                        break;
                    case "hashTableApproach":
                        result = solution.hashTableApproach(testInputs[i]);
                        break;
                    default:
                        result = 0;
                }
                
                assert result == expectedResults[i] : 
                    "Test case " + (i + 1) + " failed for " + approach;
                System.out.println("Test case " + (i + 1) + " passed");
            }
            System.out.println("\n");
        }
    }
} 