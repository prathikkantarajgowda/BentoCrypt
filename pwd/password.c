#include <shadow.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static char *progname = "password";

int
main(int argc, char **argv)
{
	if (argc != 2) {
		(void)printf("usage: sudo %s username\n", progname);
		exit(EXIT_FAILURE);
	}

	struct spwd *shadow_pwd = getspnam(argv[1]);

	if (!shadow_pwd) {
		(void)printf("failed to find records on user %s\n", argv[1]);
		exit(EXIT_FAILURE);
	}

	(void)printf("username: %s\n", shadow_pwd->sp_namp);
	(void)printf("encrypted password: %s\n", shadow_pwd->sp_pwdp);
}
