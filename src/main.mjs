class FindLadders {
  constructor (beginWord, endWord, wordList) {
    this.beginWord = beginWord
    this.endWord = endWord
    this.wordSet = new Set(wordList)
    this.distanceMap = new Map()
    this.wordMap = new Map()

    this.wordSet.add(beginWord)
  }

  solution () {
    if (!this.wordSet.has(this.endWord)) {
      return []
    }

    const reached = this.#bfs()
    if (!reached) {
      return []
    }

    const result = []
    this.#dfs(result, [this.beginWord], this.beginWord)
    return result
  }

  #bfs () {
    const beginWord = this.beginWord
    const endWord = this.endWord
    const distanceMap = this.distanceMap
    const wordMap = this.wordMap

    const queue = [endWord]
    const visited = new Set([endWord])
    let reached = false
    let distance = 0
    distanceMap.set(endWord, distance)

    while (queue.length > 0) {
      const qcount = queue.length
      distance += 1
      for (let i = 0; i < qcount; i += 1) {
        const word = queue.shift()
        for (const nextWord of this.#getNextWord(word)) {
          if (!wordMap.has(nextWord)) {
            wordMap.set(nextWord, [])
          }
          wordMap.get(nextWord).push(word)

          if (visited.has(nextWord)) {
            continue
          }

          if (nextWord === beginWord) {
            reached = true
          }

          distanceMap.set(nextWord, distance)
          queue.push(nextWord)
          visited.add(nextWord)
        }
      }
    }

    return reached
  }

  #dfs (result, tempPaths, word) {
    if (word === this.endWord) {
      result.push([...tempPaths])
      return
    }

    const distanceMap = this.distanceMap
    for (const nextWord of this.wordMap.get(word)) {
      if (distanceMap.get(word) === distanceMap.get(nextWord) + 1) {
        tempPaths.push(nextWord)
        this.#dfs(result, tempPaths, nextWord)
        tempPaths.pop()
      }
    }
  }

  #getNextWord (word) {
    const AZ = 'az'
    const ACODE = AZ.charCodeAt(0)
    const ZCODE = AZ.charCodeAt(1)
    const result = []
    for (let i = 0; i < word.length; i += 1) {
      const code = word.charCodeAt(i)
      for (let ch = ACODE; ch <= ZCODE; ch += 1) {
        if (code === ch) {
          continue
        }
        const chars = word.split('')
        chars[i] = String.fromCharCode(ch)
        const newWord = chars.join('')
        if (this.wordSet.has(newWord)) {
          result.push(newWord)
        }
      }
    }

    return result
  }
}

/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {string[][]}
 */
function findLadders (beginWord, endWord, wordList) {
  const self = new FindLadders(beginWord, endWord, wordList)
  return self.solution()
}

async function main () {
  const inputs = [
    {
      beginWord: 'hit',
      endWord: 'cog',
      wordList: ['hot', 'dot', 'dog', 'lot', 'log', 'cog']
    },
    {
      beginWord: 'hit',
      endWord: 'cog',
      wordList: ['hot', 'dot', 'dog', 'lot', 'log']
    },
    {
      beginWord: 'aaaaa',
      endWord: 'ggggg',
      wordList: ['aaaaa', 'caaaa', 'cbaaa', 'daaaa', 'dbaaa', 'eaaaa', 'ebaaa', 'faaaa', 'fbaaa', 'gaaaa', 'gbaaa', 'haaaa', 'hbaaa', 'iaaaa', 'ibaaa', 'jaaaa', 'jbaaa', 'kaaaa', 'kbaaa', 'laaaa', 'lbaaa', 'maaaa', 'mbaaa', 'naaaa', 'nbaaa', 'oaaaa', 'obaaa', 'paaaa', 'pbaaa', 'bbaaa', 'bbcaa', 'bbcba', 'bbdaa', 'bbdba', 'bbeaa', 'bbeba', 'bbfaa', 'bbfba', 'bbgaa', 'bbgba', 'bbhaa', 'bbhba', 'bbiaa', 'bbiba', 'bbjaa', 'bbjba', 'bbkaa', 'bbkba', 'bblaa', 'bblba', 'bbmaa', 'bbmba', 'bbnaa', 'bbnba', 'bboaa', 'bboba', 'bbpaa', 'bbpba', 'bbbba', 'abbba', 'acbba', 'dbbba', 'dcbba', 'ebbba', 'ecbba', 'fbbba', 'fcbba', 'gbbba', 'gcbba', 'hbbba', 'hcbba', 'ibbba', 'icbba', 'jbbba', 'jcbba', 'kbbba', 'kcbba', 'lbbba', 'lcbba', 'mbbba', 'mcbba', 'nbbba', 'ncbba', 'obbba', 'ocbba', 'pbbba', 'pcbba', 'ccbba', 'ccaba', 'ccaca', 'ccdba', 'ccdca', 'cceba', 'cceca', 'ccfba', 'ccfca', 'ccgba', 'ccgca', 'cchba', 'cchca', 'cciba', 'ccica', 'ccjba', 'ccjca', 'cckba', 'cckca', 'cclba', 'cclca', 'ccmba', 'ccmca', 'ccnba', 'ccnca', 'ccoba', 'ccoca', 'ccpba', 'ccpca', 'cccca', 'accca', 'adcca', 'bccca', 'bdcca', 'eccca', 'edcca', 'fccca', 'fdcca', 'gccca', 'gdcca', 'hccca', 'hdcca', 'iccca', 'idcca', 'jccca', 'jdcca', 'kccca', 'kdcca', 'lccca', 'ldcca', 'mccca', 'mdcca', 'nccca', 'ndcca', 'occca', 'odcca', 'pccca', 'pdcca', 'ddcca', 'ddaca', 'ddada', 'ddbca', 'ddbda', 'ddeca', 'ddeda', 'ddfca', 'ddfda', 'ddgca', 'ddgda', 'ddhca', 'ddhda', 'ddica', 'ddida', 'ddjca', 'ddjda', 'ddkca', 'ddkda', 'ddlca', 'ddlda', 'ddmca', 'ddmda', 'ddnca', 'ddnda', 'ddoca', 'ddoda', 'ddpca', 'ddpda', 'dddda', 'addda', 'aedda', 'bddda', 'bedda', 'cddda', 'cedda', 'fddda', 'fedda', 'gddda', 'gedda', 'hddda', 'hedda', 'iddda', 'iedda', 'jddda', 'jedda', 'kddda', 'kedda', 'lddda', 'ledda', 'mddda', 'medda', 'nddda', 'nedda', 'oddda', 'oedda', 'pddda', 'pedda', 'eedda', 'eeada', 'eeaea', 'eebda', 'eebea', 'eecda', 'eecea', 'eefda', 'eefea', 'eegda', 'eegea', 'eehda', 'eehea', 'eeida', 'eeiea', 'eejda', 'eejea', 'eekda', 'eekea', 'eelda', 'eelea', 'eemda', 'eemea', 'eenda', 'eenea', 'eeoda', 'eeoea', 'eepda', 'eepea', 'eeeea', 'ggggg', 'agggg', 'ahggg', 'bgggg', 'bhggg', 'cgggg', 'chggg', 'dgggg', 'dhggg', 'egggg', 'ehggg', 'fgggg', 'fhggg', 'igggg', 'ihggg', 'jgggg', 'jhggg', 'kgggg', 'khggg', 'lgggg', 'lhggg', 'mgggg', 'mhggg', 'ngggg', 'nhggg', 'ogggg', 'ohggg', 'pgggg', 'phggg', 'hhggg', 'hhagg', 'hhahg', 'hhbgg', 'hhbhg', 'hhcgg', 'hhchg', 'hhdgg', 'hhdhg', 'hhegg', 'hhehg', 'hhfgg', 'hhfhg', 'hhigg', 'hhihg', 'hhjgg', 'hhjhg', 'hhkgg', 'hhkhg', 'hhlgg', 'hhlhg', 'hhmgg', 'hhmhg', 'hhngg', 'hhnhg', 'hhogg', 'hhohg', 'hhpgg', 'hhphg', 'hhhhg', 'ahhhg', 'aihhg', 'bhhhg', 'bihhg', 'chhhg', 'cihhg', 'dhhhg', 'dihhg', 'ehhhg', 'eihhg', 'fhhhg', 'fihhg', 'ghhhg', 'gihhg', 'jhhhg', 'jihhg', 'khhhg', 'kihhg', 'lhhhg', 'lihhg', 'mhhhg', 'mihhg', 'nhhhg', 'nihhg', 'ohhhg', 'oihhg', 'phhhg', 'pihhg', 'iihhg', 'iiahg', 'iiaig', 'iibhg', 'iibig', 'iichg', 'iicig', 'iidhg', 'iidig', 'iiehg', 'iieig', 'iifhg', 'iifig', 'iighg', 'iigig', 'iijhg', 'iijig', 'iikhg', 'iikig', 'iilhg', 'iilig', 'iimhg', 'iimig', 'iinhg', 'iinig', 'iiohg', 'iioig', 'iiphg', 'iipig', 'iiiig', 'aiiig', 'ajiig', 'biiig', 'bjiig', 'ciiig', 'cjiig', 'diiig', 'djiig', 'eiiig', 'ejiig', 'fiiig', 'fjiig', 'giiig', 'gjiig', 'hiiig', 'hjiig', 'kiiig', 'kjiig', 'liiig', 'ljiig', 'miiig', 'mjiig', 'niiig', 'njiig', 'oiiig', 'ojiig', 'piiig', 'pjiig', 'jjiig', 'jjaig', 'jjajg', 'jjbig', 'jjbjg', 'jjcig', 'jjcjg', 'jjdig', 'jjdjg', 'jjeig', 'jjejg', 'jjfig', 'jjfjg', 'jjgig', 'jjgjg', 'jjhig', 'jjhjg', 'jjkig', 'jjkjg', 'jjlig', 'jjljg', 'jjmig', 'jjmjg', 'jjnig', 'jjnjg', 'jjoig', 'jjojg', 'jjpig', 'jjpjg', 'jjjjg', 'ajjjg', 'akjjg', 'bjjjg', 'bkjjg', 'cjjjg', 'ckjjg', 'djjjg', 'dkjjg', 'ejjjg', 'ekjjg', 'fjjjg', 'fkjjg', 'gjjjg', 'gkjjg', 'hjjjg', 'hkjjg', 'ijjjg', 'ikjjg', 'ljjjg', 'lkjjg', 'mjjjg', 'mkjjg', 'njjjg', 'nkjjg', 'ojjjg', 'okjjg', 'pjjjg', 'pkjjg', 'kkjjg', 'kkajg', 'kkakg', 'kkbjg', 'kkbkg', 'kkcjg', 'kkckg', 'kkdjg', 'kkdkg', 'kkejg', 'kkekg', 'kkfjg', 'kkfkg', 'kkgjg', 'kkgkg', 'kkhjg', 'kkhkg', 'kkijg', 'kkikg', 'kkljg', 'kklkg', 'kkmjg', 'kkmkg', 'kknjg', 'kknkg', 'kkojg', 'kkokg', 'kkpjg', 'kkpkg', 'kkkkg', 'ggggx', 'gggxx', 'ggxxx', 'gxxxx', 'xxxxx', 'xxxxy', 'xxxyy', 'xxyyy', 'xyyyy', 'yyyyy', 'yyyyw', 'yyyww', 'yywww', 'ywwww', 'wwwww', 'wwvww', 'wvvww', 'vvvww', 'vvvwz', 'avvwz', 'aavwz', 'aaawz', 'aaaaz']
    }
  ]

  for (const { beginWord, endWord, wordList } of inputs) {
    const result = findLadders(beginWord, endWord, wordList)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
