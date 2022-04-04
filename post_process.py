import statistics as stats

confidenceThreshold = 300

file1 = open('res', 'r')
lines = file1.readlines()

def mapInt(num):
    return int(num)

def printThresh(l, threshold):
    outliers = list(filter(lambda n: n > threshold, l))
    threshCount = len(outliers)
    print("Values above " + str(threshold))
    print("     Number: " + str(threshCount))
    print("     Confidence: " + str(threshCount) + "/" + str(count) + " | or " + str(threshCount/count))


lines = list(map(mapInt, lines))
count = len(lines)
total = sum(lines)
top = max(lines)

    
print("Num Measurements: " + str(count))
print("Avg: " + str(stats.mean(lines)))
print("Max: " + str(top))

printThresh(lines, 200)
printThresh(lines, 300)
printThresh(lines, 500)