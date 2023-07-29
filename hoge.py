print("""<?xml version="1.0" encoding="UTF-8"?>
<Run version="1.7.0">
  <GameIcon />
  <GameName>ABC2xx B</GameName>
  <CategoryName>
  </CategoryName>
  <LayoutPath>
  </LayoutPath>
  <Metadata>
    <Run id="" />
    <Platform usesEmulator="False">
    </Platform>
    <Region>
    </Region>
    <Variables />
  </Metadata>
  <Offset>00:00:00</Offset>
  <AttemptCount>0</AttemptCount>
  <AttemptHistory />
  <Segments>
""")
for i in range(200, 300):
    print(f"""    <Segment>
      <Name>{i}</Name>
      <Icon />
      <SplitTimes>
        <SplitTime name="Personal Best" />
      </SplitTimes>
      <BestSegmentTime />
      <SegmentHistory />
    </Segment>""")

print("""  </Segments>
  <AutoSplitterSettings />
</Run>""")
