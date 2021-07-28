# 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
#  你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
#
#
#
#  示例:
#
#  给定 nums = [2, 7, 11, 15], target = 9
#
# 因为 nums[0] + nums[1] = 2 + 7 = 9
# 所以返回 [0, 1]
#
#  Related Topics 数组 哈希表
#  👍 8749 👎 0


# leetcode submit region begin(Prohibit modification and deletion)
class Solution(object):
    # noinspection PyMethodMayBeStatic
    def twosum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        for i in nums:
            remainder = target - i
         
            if remainder in nums:
                if i != remainder:
                    return [nums.index(i), nums.index(remainder)]

                nums_copy = nums[:]
                nums_copy.pop(nums.index(i))
                if i == remainder and i in nums_copy:
                    return [nums.index(i), nums_copy.index(i - 1)]


# leetcode submit region end(Prohibit modification and deletion)
a = Solution()
print(a.twosum(nums=[3, 3], target=6))
