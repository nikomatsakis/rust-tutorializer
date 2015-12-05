#!/usr/bin/env python

import sys
import re
from urllib import urlencode

href = re.compile(r'href="src/([a-z_]+).rs"')
start_solution = re.compile(r' *// START SOLUTION$')
end_solution = re.compile(r' *// END SOLUTION$')
prompt = re.compile(r'// PROMPT ')

def replace(mo):
    path = mo.group(1)
    file_lines = file("src/%s.rs" % path).read().split('\n')
    state = 'keep'
    output_lines = []
    for line in file_lines:
        if state == 'skip':
            if end_solution.match(line):
                state = 'keep'
            elif start_solution.match(line):
                raise "Unmatched START SOLUTION in %s" % path
        elif start_solution.match(line):
            state = 'skip'
        elif end_solution.match(line):
            raise "Unmatched END SOLUTION in %s" % Path
        else:
            line = prompt.sub("", line)
            output_lines.append(line)
    file_contents = '\n'.join(output_lines)
    query = urlencode({"version": "stable", "code": file_contents})
    return 'href="https://play.rust-lang.org/?%s"' % query
                            
for line in sys.stdin:
    line2 = href.sub(replace, line)
    sys.stdout.write(line2)
    
