# how to af falke 21-02-22:
#   - vent indtil 3/4. semester har fået grupperum og derved mail-alias
#   - søg på moodle efter dat/sw/ixd+3/4, enroll og tjek hvor mange grupper der er, e.g. for 42 dat grupper, input tuplen ("dat", 1, 42)
#   - input codes i perioden som der ønskes dukse (typisk fra nu til projektaflevering) fra f-server (/data/cronjobs/Automailing/lstfiles/noegle.lst)
#   - sæt startdato (søndag hvorved nøglemail kommer) og næstkommende ugenummer (ugennummer én dag efter nøglemail udkommer)
#   - validér mailoutput ved at smide en mail i din outlookklient som modtager på en mail og tryk ctrl+k for at se om alias eksistere, og om modtagere ser valide ud
#   - hvis alt ok, så postfix script output til '/data/cronjobs/Automailing/listfiles/dukse.lst'

import random
from random import shuffle
from datetime import datetime, timedelta

groups = [
    ("sw", 1, 20),
    ("dat", 1, 7)]
codes = [
    8735,
    2319,
    7063,
    7040,
    5761,
    9044,
    7821,
    7923,
    7100,
    3750,
    8662,
    8437,
    1279,
    4480]

prefix = "cs-24-"
semester = '3'

mailpostfix = "@student.aau.dk"  # changed from @cs.aau.dk during summer '21

weeksToGenerate = len(codes)
weekStart = 8
startDate = datetime(2022, 2, 20)

mails = []

for group in groups:
    for x in range(group[1], group[2] + 1):
        if(x == 12):
            mails.append(group[0] + '-' + semester + '-' + '13')  # mails have leading zero
        else:
            mails.append(group[0] + '-' + semester + '-' + f'{x:02d}')  # mails have leading zero
            

shuffle(mails)

mails *= int((weeksToGenerate * 2) / len(mails) + 1)

for i in range(0, weeksToGenerate):
    pair = mails[0:2]
    mails = mails[2:]
    date = startDate + timedelta(days=i * 7)
    line = "{}|{}|{}|{}|{}".format(
        date.strftime("%Y-%m-%d"),
        ";".join(map(lambda x: prefix + x + mailpostfix, pair))
        + ";treo@fklub.dk",
        "treo@fklub.dk",
        "Duksegruppe",
        "GROUP=" + " og ".join(map(lambda x: prefix + x, pair)) + ";WEEK=" + str(weekStart
                                                                                 + i) + ";CODE=" + str(codes[i]).zfill(
            4)
    )
