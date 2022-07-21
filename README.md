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
4.  字符串的匹配 (str_pratice_wildcard_match_recursion)

5. 字符串空格的压缩 (str_practice_space_compress)

11. 最小子串 (sub_string_in_file)
12. 最长连续字符 (most_repeat_alphabet_recur)