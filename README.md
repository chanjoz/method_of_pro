# Python和Rust算法练习- 编程之法

---

包含了[**编程之法** 面试和算法心得 July著] 一书中算法示例和习题的Python和Rust实现。 (原书代码为C++)

## 第一章 字符串

1.1 字符串的旋转 

解法二: 三步反转 	 (3_step_inverse)

将字符串分割成两个部分，然后将这两个部分的字符串分别反转，最后再对整个字符串进行整体反转

1.2 字符串的全排列

解法二：字典序排列 (next_permutation)

1. 找到排列中最后（最右）一个升序的首个位置i,  x=a<sub>i</sub>

2. 找到排列中第i位右边最后一个比a<sub>i</sub> 大的位置j, y=a<sub>j</sub>

3. 交换x和y

4. 把第i+1位到最后的部分翻转

   e.g.  21543

   - x=1

   - y=3

   - 1和3 交换 23541

   - 翻转541， 得23145

1.4 字符串转换成整数 

(str_num)

1.5 回文判断

解法一： 两头往中间扫 (check_palindrome)

### 习题

2. 字符串的左右移动 (str_practice_mv_asterisk)
3. 字符串个数的统计 (str_practice_count_frequency_alpha)
4. 字符串的匹配 (str_pratice_wildcard_match_recursion)
5. 字符串空格的压缩 (str_practice_space_compress)
11. 最小子串 (sub_string_in_file)
12. 最长连续字符 (most_repeat_alphabet_recur)



## 第二章 数组

2.1  寻找最小的k个数

解法二： 部分排序 (min_5_in_array)

解法三： 用堆代替数组 (min_5_in_array_optim_maxheap)

解法四： 线性选择算法 (quick_select)

从数组中选取“中位数的中位数”作为主元， 把数组划分为s<sub>a</sub>和s<sub>b</sub>两部分， 如果k小于s<sub>a</sub>的个数， 对集合s<sub>a</sub>

继续迭代quick_select算法， 如果反之， 返回s<sub>a</sub>的所有元素，和s<sub>b</sub>中较小的k-|S<sub>a</sub>|个元素。

2.2 寻找和为定值的两个数

解法二： 排序夹逼 (array_find_sum_1)

先排序，然后数组收尾夹逼

2.7 荷兰国旗问题 举一反三 RGB 三字符重排列

(rgb_sort)

### 习题

1. 唯一的重复元素 

​	解法一： 数组索引与值匹配互换 (only_repeat_num)

3. 5. 两个只出现一次（奇数此）的数字a, b (no_repeat_num)

​    解法：所有数异或得到两个出现一次数的和x = a^b, 以x二机制值最后一位为1的位置判断数组所有元素

将数组分成两部分， 对第一部分全部异或都到a， a^x =b













