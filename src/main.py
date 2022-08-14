"""
main
"""

from __future__ import annotations
from typing import Optional
from queue import Queue

class Solution:
    begin_word: str
    end_word: str
    word_set: set[str]
    distance_map: dict[str, int]
    word_map: dict[str, list[str]]


    def findLadders(self, begin_word: str, end_word: str, word_list: list[str]) -> list[list[str]]:
        self.begin_word = begin_word
        self.end_word = end_word
        self.word_set = set(word_list)
        self.distance_map = {}
        self.word_map = {}

        self.word_set.add(begin_word)

        if end_word not in self.word_set:
            return []

        reached = self.bfs()
        if not reached:
            return []

        result: list[list[str]] = []
        self.dfs(result, [begin_word], begin_word)

        return result

    def dfs(self, result: list[list[str]], temp_paths: list[str], word: str):
        if word == self.end_word:
            result.append(temp_paths.copy())
            return

        next_words = self.word_map.get(word)
        if next_words is None:
            return

        distance_map = self.distance_map
        for next_word in next_words:
            wdistance = distance_map.get(word)
            ndistance = distance_map.get(next_word)
            if wdistance is None or ndistance is None:
                continue

            if wdistance == (ndistance + 1):
                temp_paths.append(next_word)
                self.dfs(result, temp_paths, next_word)
                temp_paths.pop()


    def bfs(self) -> bool:
        begin_word = self.begin_word
        end_word = self.end_word
        distance_map = self.distance_map
        word_map = self.word_map

        queue: Queue[str] = Queue()
        visited: set[str] = set()
        reached = False
        distance = 0

        distance_map[end_word] = distance
        queue.put(end_word)
        visited.add(end_word)
        while not queue.empty():
            qcount = queue.qsize()
            distance += 1
            for _ in range(qcount):
                word = queue.get()
                for next_word in self.get_next_word(word):
                    if next_word not in word_map:
                        word_map[next_word] = []
                    word_map[next_word].append(word)

                    if next_word in visited:
                        continue

                    if next_word == begin_word:
                        reached = True

                    distance_map[next_word] = distance
                    queue.put(next_word)
                    visited.add(next_word)


        return reached

    def get_next_word(self, word: str) -> list[str]:
        ACODE = ord('a')
        ZCODE = ord('z')

        result: list[str] = []
        for i in range(len(word)):
            code = ord(word[i])
            for ch in range(ACODE, ZCODE + 1):
                if ch == code:
                    continue

                chars = list(word)
                chars[i] = chr(ch)
                new_word = "".join(chars)
                if new_word in self.word_set:
                    result.append(new_word)

        return result



class Input:
    begin_word: str
    end_word: str
    word_list: list[str]

    def __init__(self, begin_word: str, end_word: str, word_list: list[str]):
        self.begin_word = begin_word
        self.end_word = end_word
        self.word_list = word_list

def main():
    inputs: list[Input] = [
            Input(
                "hit",
                "cog",
                ["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            Input(
                "hit",
                "cog",
                ["hot", "dot", "dog", "lot", "log"]
            ),
            Input(
                "aaaaa",
                "ggggg",
                ["aaaaa", "caaaa", "cbaaa", "daaaa", "dbaaa", "eaaaa", "ebaaa", "faaaa", "fbaaa", "gaaaa", "gbaaa", "haaaa", "hbaaa", "iaaaa", "ibaaa", "jaaaa", "jbaaa", "kaaaa", "kbaaa", "laaaa", "lbaaa", "maaaa", "mbaaa", "naaaa", "nbaaa", "oaaaa", "obaaa", "paaaa", "pbaaa", "bbaaa", "bbcaa", "bbcba", "bbdaa", "bbdba", "bbeaa", "bbeba", "bbfaa", "bbfba", "bbgaa", "bbgba", "bbhaa", "bbhba", "bbiaa", "bbiba", "bbjaa", "bbjba", "bbkaa", "bbkba", "bblaa", "bblba", "bbmaa", "bbmba", "bbnaa", "bbnba", "bboaa", "bboba", "bbpaa", "bbpba", "bbbba", "abbba", "acbba", "dbbba", "dcbba", "ebbba", "ecbba", "fbbba", "fcbba", "gbbba", "gcbba", "hbbba", "hcbba", "ibbba", "icbba", "jbbba", "jcbba", "kbbba", "kcbba", "lbbba", "lcbba", "mbbba", "mcbba", "nbbba", "ncbba", "obbba", "ocbba", "pbbba", "pcbba", "ccbba", "ccaba", "ccaca", "ccdba", "ccdca", "cceba", "cceca", "ccfba", "ccfca", "ccgba", "ccgca", "cchba", "cchca", "cciba", "ccica", "ccjba", "ccjca", "cckba", "cckca", "cclba", "cclca", "ccmba", "ccmca", "ccnba", "ccnca", "ccoba", "ccoca", "ccpba", "ccpca", "cccca", "accca", "adcca", "bccca", "bdcca", "eccca", "edcca", "fccca", "fdcca", "gccca", "gdcca", "hccca", "hdcca", "iccca", "idcca", "jccca", "jdcca", "kccca", "kdcca", "lccca", "ldcca", "mccca", "mdcca", "nccca", "ndcca", "occca", "odcca", "pccca", "pdcca", "ddcca", "ddaca", "ddada", "ddbca", "ddbda", "ddeca", "ddeda", "ddfca", "ddfda", "ddgca", "ddgda", "ddhca", "ddhda", "ddica", "ddida", "ddjca", "ddjda", "ddkca", "ddkda", "ddlca", "ddlda", "ddmca", "ddmda", "ddnca", "ddnda", "ddoca", "ddoda", "ddpca", "ddpda", "dddda", "addda", "aedda", "bddda", "bedda", "cddda", "cedda", "fddda", "fedda", "gddda", "gedda", "hddda", "hedda", "iddda", "iedda", "jddda", "jedda", "kddda", "kedda", "lddda", "ledda", "mddda", "medda", "nddda", "nedda", "oddda", "oedda", "pddda", "pedda", "eedda", "eeada", "eeaea", "eebda", "eebea", "eecda", "eecea", "eefda", "eefea", "eegda", "eegea", "eehda", "eehea", "eeida", "eeiea", "eejda", "eejea", "eekda", "eekea", "eelda", "eelea", "eemda", "eemea", "eenda", "eenea", "eeoda", "eeoea", "eepda", "eepea", "eeeea", "ggggg", "agggg", "ahggg", "bgggg", "bhggg", "cgggg", "chggg", "dgggg", "dhggg", "egggg", "ehggg", "fgggg", "fhggg", "igggg", "ihggg", "jgggg", "jhggg", "kgggg", "khggg", "lgggg", "lhggg", "mgggg", "mhggg", "ngggg", "nhggg", "ogggg", "ohggg", "pgggg", "phggg", "hhggg", "hhagg", "hhahg", "hhbgg", "hhbhg", "hhcgg", "hhchg", "hhdgg", "hhdhg", "hhegg", "hhehg", "hhfgg", "hhfhg", "hhigg", "hhihg", "hhjgg", "hhjhg", "hhkgg", "hhkhg", "hhlgg", "hhlhg", "hhmgg", "hhmhg", "hhngg", "hhnhg", "hhogg", "hhohg", "hhpgg", "hhphg", "hhhhg", "ahhhg", "aihhg", "bhhhg", "bihhg", "chhhg", "cihhg", "dhhhg", "dihhg", "ehhhg", "eihhg", "fhhhg", "fihhg", "ghhhg", "gihhg", "jhhhg", "jihhg", "khhhg", "kihhg", "lhhhg", "lihhg", "mhhhg", "mihhg", "nhhhg", "nihhg", "ohhhg", "oihhg", "phhhg", "pihhg", "iihhg", "iiahg", "iiaig", "iibhg", "iibig", "iichg", "iicig", "iidhg", "iidig", "iiehg", "iieig", "iifhg", "iifig", "iighg", "iigig", "iijhg", "iijig", "iikhg", "iikig", "iilhg", "iilig", "iimhg", "iimig", "iinhg", "iinig", "iiohg", "iioig", "iiphg", "iipig", "iiiig", "aiiig", "ajiig", "biiig", "bjiig", "ciiig", "cjiig", "diiig", "djiig", "eiiig", "ejiig", "fiiig", "fjiig", "giiig", "gjiig", "hiiig", "hjiig", "kiiig", "kjiig", "liiig", "ljiig", "miiig", "mjiig", "niiig", "njiig", "oiiig", "ojiig", "piiig", "pjiig", "jjiig", "jjaig", "jjajg", "jjbig", "jjbjg", "jjcig", "jjcjg", "jjdig", "jjdjg", "jjeig", "jjejg", "jjfig", "jjfjg", "jjgig", "jjgjg", "jjhig", "jjhjg", "jjkig", "jjkjg", "jjlig", "jjljg", "jjmig", "jjmjg", "jjnig", "jjnjg", "jjoig", "jjojg", "jjpig", "jjpjg", "jjjjg", "ajjjg", "akjjg", "bjjjg", "bkjjg", "cjjjg", "ckjjg", "djjjg", "dkjjg", "ejjjg", "ekjjg", "fjjjg", "fkjjg", "gjjjg", "gkjjg", "hjjjg", "hkjjg", "ijjjg", "ikjjg", "ljjjg", "lkjjg", "mjjjg", "mkjjg", "njjjg", "nkjjg", "ojjjg", "okjjg", "pjjjg", "pkjjg", "kkjjg", "kkajg", "kkakg", "kkbjg", "kkbkg", "kkcjg", "kkckg", "kkdjg", "kkdkg", "kkejg", "kkekg", "kkfjg", "kkfkg", "kkgjg", "kkgkg", "kkhjg", "kkhkg", "kkijg", "kkikg", "kkljg", "kklkg", "kkmjg", "kkmkg", "kknjg", "kknkg", "kkojg", "kkokg", "kkpjg", "kkpkg", "kkkkg", "ggggx", "gggxx", "ggxxx", "gxxxx", "xxxxx", "xxxxy", "xxxyy", "xxyyy", "xyyyy", "yyyyy", "yyyyw", "yyyww", "yywww", "ywwww", "wwwww", "wwvww", "wvvww", "vvvww", "vvvwz", "avvwz", "aavwz", "aaawz", "aaaaz"]
            ),
    ]

    s = Solution()
    for i in inputs:
        result = s.findLadders(i.begin_word, i.end_word, i.word_list)
        print(result)

if __name__ == "__main__":
    main()
