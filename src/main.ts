import '@total-typescript/ts-reset'

class UndergroundSystem {
  private readonly times: Map<string, [number, number]>
  private readonly checkIns: Map<number, [string, number]>
  public constructor () {
    this.times = new Map()
    this.checkIns = new Map()
  }

  public checkIn (id: number, stationName: string, t: number): void {
    this.checkIns.set(id, [stationName, t])
  }

  public checkOut (id: number, stationName: string, t: number): void {
    const memo = this.checkIns.get(id)
    if (memo == null) {
      return
    }

    const [from, inAt] = memo
    const key = `${from}-${stationName}`
    const travelTime = t - inAt
    const [total, count] = this.times.get(key) ?? [0, 0]
    this.times.set(key, [total + travelTime, count + 1])
  }

  public getAverageTime (startStation: string, endStation: string): number {
    const key = `${startStation}-${endStation}`
    const [total, count] = this.times.get(key) ?? [0, 0]
    return total / count
  }
}

function case0 (): void {
  const undergroundSystem = new UndergroundSystem()
  undergroundSystem.checkIn(45, 'Leyton', 3)
  undergroundSystem.checkIn(32, 'Paradise', 8)
  undergroundSystem.checkIn(27, 'Leyton', 10)
  undergroundSystem.checkOut(45, 'Waterloo', 15)
  undergroundSystem.checkOut(27, 'Waterloo', 20)
  undergroundSystem.checkOut(32, 'Cambridge', 22)
  console.log(undergroundSystem.getAverageTime('Paradise', 'Cambridge'))
  console.log(undergroundSystem.getAverageTime('Leyton', 'Waterloo'))
  undergroundSystem.checkIn(10, 'Leyton', 24)
  console.log(undergroundSystem.getAverageTime('Leyton', 'Waterloo'))
  undergroundSystem.checkOut(10, 'Waterloo', 38)
  console.log(undergroundSystem.getAverageTime('Leyton', 'Waterloo'))
}

function case1 (): void {
  const undergroundSystem = new UndergroundSystem()
  undergroundSystem.checkIn(10, 'Leyton', 3)
  undergroundSystem.checkOut(10, 'Paradise', 8)
  console.log(undergroundSystem.getAverageTime('Leyton', 'Paradise'))
  undergroundSystem.checkIn(5, 'Leyton', 10)
  undergroundSystem.checkOut(5, 'Paradise', 16)
  console.log(undergroundSystem.getAverageTime('Leyton', 'Paradise'))
  undergroundSystem.checkIn(2, 'Leyton', 21)
  undergroundSystem.checkOut(2, 'Paradise', 30)
  console.log(undergroundSystem.getAverageTime('Leyton', 'Paradise'))
}

function main (): void {
  case0()
  console.log('---')
  case1()
}
main()
