#!/usr/bin/env python

import sys
import re
from urllib import urlencode

href = re.compile(r'href="src/([-a-z_]+).rs"')
start_solution = re.compile(r' *// START SOLUTION$')
end_solution = re.compile(r' *// END SOLUTION$')
prompt = re.compile(r'// PROMPT ')
home_url_placeholder = "http://home.url"

if len(sys.argv) <= 1:
    sys.stderr.write("Missing home-url argument\n");
    sys.exit(1)

home_url = sys.argv[1]

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
                raise Exception("Unmatched START SOLUTION in %s" % path)
        elif start_solution.match(line):
            state = 'skip'
        elif end_solution.match(line):
            raise Exception("Unmatched END SOLUTION in %s" % path)
        else:
            line = prompt.sub("", line)
            line = line.replace(home_url_placeholder, home_url)
            output_lines.append(line)
    file_contents = '\n'.join(output_lines)
    query = urlencode({"version": "nightly", "code": file_contents})
    return 'href="https://play.rust-lang.org/?%s"' % query
                            
for line in sys.stdin:
    line = href.sub(replace, line)
    line = line.replace(home_url_placeholder, home_url)
    sys.stdout.write(line)
    
