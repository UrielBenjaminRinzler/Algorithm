package com.sec01.ex230312

class TwoSum { }
class Solution {
    fun twoSum(nums:IntArray, target:Int):IntArray {
        for(i in 0..nums.size-1)
            for(j in i+1..nums.size-1)
                if(nums[i] + nums[j] == target) return intArrayOf(i, j)
        return intArrayOf(-1, 0)
    }
}

fun main() {
    var sol = Solution()
    var result:IntArray = sol.twoSum(intArrayOf(2, 7, 11, 15), 9)
    for(res in result) println(res)
}